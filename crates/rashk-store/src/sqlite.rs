use async_trait::async_trait;
use rashk_core::error::RashkError;
use rashk_core::ports::Store;
use rashk_core::types::{Query, Record, RecordId};
use rusqlite::Connection;
use std::sync::Mutex;

pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    pub fn open(path: &str) -> Result<Self, RashkError> {
        let conn = if path == ":memory:" {
            Connection::open_in_memory()
        } else {
            Connection::open(path)
        }
        .map_err(|e| RashkError::Store(e.to_string()))?;

        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;
             PRAGMA foreign_keys=ON;",
        )
        .map_err(|e| RashkError::Store(e.to_string()))?;

        let store = Self {
            conn: Mutex::new(conn),
        };
        store.migrate()?;
        Ok(store)
    }

    pub fn memory() -> Result<Self, RashkError> {
        Self::open(":memory:")
    }

    fn migrate(&self) -> Result<(), RashkError> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS records (
                id          TEXT PRIMARY KEY,
                kind        TEXT NOT NULL,
                data        TEXT NOT NULL,
                metadata    TEXT NOT NULL DEFAULT '{}',
                created_at  TEXT NOT NULL,
                updated_at  TEXT NOT NULL,
                origin_node TEXT,
                hlc         INTEGER NOT NULL DEFAULT 0
            );
            CREATE INDEX IF NOT EXISTS idx_records_kind ON records(kind);
            CREATE INDEX IF NOT EXISTS idx_records_hlc ON records(hlc);

            CREATE TABLE IF NOT EXISTS blobs (
                key          TEXT PRIMARY KEY,
                data         BLOB NOT NULL,
                content_type TEXT NOT NULL,
                name         TEXT,
                created_at   TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE VIRTUAL TABLE IF NOT EXISTS search_index USING fts5(
                record_id, kind, content
            );",
        )
        .map_err(|e| RashkError::Store(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl Store for SqliteStore {
    async fn get(&self, id: &RecordId) -> Result<Option<Record>, RashkError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT id, kind, data, metadata, created_at, updated_at, origin_node, hlc FROM records WHERE id = ?1")
            .map_err(|e| RashkError::Store(e.to_string()))?;

        let result = stmt
            .query_row(rusqlite::params![id.0], |row| {
                Ok(Record {
                    id: RecordId(row.get::<_, String>(0)?),
                    kind: row.get(1)?,
                    data: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    metadata: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                    created_at: row
                        .get::<_, String>(4)?
                        .parse()
                        .unwrap_or_default(),
                    updated_at: row
                        .get::<_, String>(5)?
                        .parse()
                        .unwrap_or_default(),
                    origin_node: row.get::<_, Option<String>>(6)?.map(rashk_core::types::NodeId),
                    hlc: row.get(7)?,
                })
            })
            .optional()
            .map_err(|e| RashkError::Store(e.to_string()))?;

        Ok(result)
    }

    async fn put(&self, record: &Record) -> Result<(), RashkError> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO records (id, kind, data, metadata, created_at, updated_at, origin_node, hlc)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                record.id.0,
                record.kind,
                serde_json::to_string(&record.data).unwrap_or_default(),
                serde_json::to_string(&record.metadata).unwrap_or_default(),
                record.created_at.to_rfc3339(),
                record.updated_at.to_rfc3339(),
                record.origin_node.as_ref().map(|n| &n.0),
                record.hlc,
            ],
        )
        .map_err(|e| RashkError::Store(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: &RecordId) -> Result<(), RashkError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM records WHERE id = ?1", rusqlite::params![id.0])
            .map_err(|e| RashkError::Store(e.to_string()))?;
        Ok(())
    }

    async fn query(&self, query: &Query) -> Result<Vec<Record>, RashkError> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from(
            "SELECT id, kind, data, metadata, created_at, updated_at, origin_node, hlc FROM records",
        );
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if let Some(kind) = &query.kind {
            sql.push_str(" WHERE kind = ?");
            params.push(Box::new(kind.clone()));
        }

        let order = match query.order_by.as_deref().unwrap_or("created_at") {
            "created_at" => "created_at",
            "updated_at" => "updated_at",
            "kind" => "kind",
            "hlc" => "hlc",
            "id" => "id",
            _ => "created_at",
        };
        let dir = if query.order_desc { "DESC" } else { "ASC" };
        sql.push_str(&format!(" ORDER BY {order} {dir}"));

        if let Some(limit) = query.limit {
            sql.push_str(" LIMIT ?");
            params.push(Box::new(limit));
        }
        if let Some(offset) = query.offset {
            sql.push_str(" OFFSET ?");
            params.push(Box::new(offset));
        }

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| RashkError::Store(e.to_string()))?;

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let rows = stmt
            .query_map(param_refs.as_slice(), |row| {
                Ok(Record {
                    id: RecordId(row.get::<_, String>(0)?),
                    kind: row.get(1)?,
                    data: serde_json::from_str(&row.get::<_, String>(2)?).unwrap_or_default(),
                    metadata: serde_json::from_str(&row.get::<_, String>(3)?).unwrap_or_default(),
                    created_at: row.get::<_, String>(4)?.parse().unwrap_or_default(),
                    updated_at: row.get::<_, String>(5)?.parse().unwrap_or_default(),
                    origin_node: row.get::<_, Option<String>>(6)?.map(rashk_core::types::NodeId),
                    hlc: row.get(7)?,
                })
            })
            .map_err(|e| RashkError::Store(e.to_string()))?;

        let mut records = Vec::new();
        for row in rows {
            records.push(row.map_err(|e| RashkError::Store(e.to_string()))?);
        }
        Ok(records)
    }

    async fn count(&self, query: &Query) -> Result<u64, RashkError> {
        let conn = self.conn.lock().unwrap();
        let mut sql = String::from("SELECT COUNT(*) FROM records");
        let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![];

        if let Some(kind) = &query.kind {
            sql.push_str(" WHERE kind = ?");
            params.push(Box::new(kind.clone()));
        }

        let param_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let count: u64 = conn
            .query_row(&sql, param_refs.as_slice(), |row| row.get(0))
            .map_err(|e| RashkError::Store(e.to_string()))?;

        Ok(count)
    }
}

use rusqlite::OptionalExtension;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_crud() {
        let store = SqliteStore::memory().unwrap();
        let record = Record::new("test.item", serde_json::json!({"name": "hello"}));
        let id = record.id.clone();

        store.put(&record).await.unwrap();
        let got = store.get(&id).await.unwrap().unwrap();
        assert_eq!(got.kind, "test.item");

        store.delete(&id).await.unwrap();
        assert!(store.get(&id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_query_by_kind() {
        let store = SqliteStore::memory().unwrap();
        store.put(&Record::new("task", serde_json::json!({"n": 1}))).await.unwrap();
        store.put(&Record::new("task", serde_json::json!({"n": 2}))).await.unwrap();
        store.put(&Record::new("note", serde_json::json!({"n": 3}))).await.unwrap();

        let tasks = store.query(&Query::by_kind("task")).await.unwrap();
        assert_eq!(tasks.len(), 2);

        let count = store.count(&Query::by_kind("note")).await.unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_query_limit_offset_order() {
        let store = SqliteStore::memory().unwrap();
        for i in 0..5 {
            let mut r = Record::new("item", serde_json::json!({"n": i}));
            r.hlc = i;
            store.put(&r).await.unwrap();
        }

        // Limit
        let q = Query { kind: Some("item".into()), limit: Some(2), ..Default::default() };
        let results = store.query(&q).await.unwrap();
        assert_eq!(results.len(), 2);

        // Offset
        let q = Query { kind: Some("item".into()), limit: Some(2), offset: Some(2), ..Default::default() };
        let results = store.query(&q).await.unwrap();
        assert_eq!(results.len(), 2);

        // Order by hlc desc
        let q = Query {
            kind: Some("item".into()),
            order_by: Some("hlc".into()),
            order_desc: true,
            limit: Some(1),
            ..Default::default()
        };
        let results = store.query(&q).await.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].hlc, 4);

        // Invalid order_by falls back to created_at (no SQL injection)
        let q = Query {
            kind: Some("item".into()),
            order_by: Some("id; DROP TABLE records;--".into()),
            ..Default::default()
        };
        let results = store.query(&q).await.unwrap();
        assert_eq!(results.len(), 5); // table still intact
    }

    #[tokio::test]
    async fn test_put_get_roundtrip_data_integrity() {
        let store = SqliteStore::memory().unwrap();
        let data = serde_json::json!({
            "name": "test",
            "nested": {"a": 1, "b": [2, 3]},
            "unicode": "hello"
        });
        let record = Record::new("complex", data.clone());
        let id = record.id.clone();

        store.put(&record).await.unwrap();
        let got = store.get(&id).await.unwrap().unwrap();
        assert_eq!(got.data, data);
        assert_eq!(got.kind, "complex");
    }
}
