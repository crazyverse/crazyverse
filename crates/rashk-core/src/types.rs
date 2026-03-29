use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Universal record ID — works local, mesh, and on-chain.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RecordId(pub String);

impl RecordId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for RecordId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for RecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A record in the store. Schema-free with typed envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    pub id: RecordId,
    pub kind: String,
    pub data: serde_json::Value,
    pub metadata: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Which node created this record (for CRDT merge)
    pub origin_node: Option<NodeId>,
    /// Lamport-like clock for conflict resolution
    pub hlc: u64,
}

impl Record {
    pub fn new(kind: impl Into<String>, data: serde_json::Value) -> Self {
        let now = Utc::now();
        Self {
            id: RecordId::new(),
            kind: kind.into(),
            data,
            metadata: serde_json::Value::Object(Default::default()),
            created_at: now,
            updated_at: now,
            origin_node: None,
            hlc: 0,
        }
    }
}

/// Identity of a node in the mesh.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    pub fn from_str(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl Default for NodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A peer on the mesh network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Peer {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub addresses: Vec<String>,
    pub capabilities: Vec<String>,
    pub last_seen: DateTime<Utc>,
}

/// Event emitted by any layer — stored, synced, replayed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: RecordId,
    pub kind: String,
    pub source: String,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub node_id: Option<NodeId>,
}

impl Event {
    pub fn new(kind: impl Into<String>, source: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            id: RecordId::new(),
            kind: kind.into(),
            source: source.into(),
            data,
            timestamp: Utc::now(),
            node_id: None,
        }
    }
}

/// Query filter for store operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Query {
    pub kind: Option<String>,
    pub filter: Option<serde_json::Value>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
    pub order_by: Option<String>,
    pub order_desc: bool,
}

impl Query {
    pub fn by_kind(kind: impl Into<String>) -> Self {
        Self {
            kind: Some(kind.into()),
            ..Default::default()
        }
    }
}

/// Byte blob with content type — for files, media, WASM modules.
#[derive(Debug, Clone)]
pub struct Blob {
    pub data: bytes::Bytes,
    pub content_type: String,
    pub name: Option<String>,
}
