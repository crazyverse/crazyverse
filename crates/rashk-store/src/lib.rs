//! rashk-store: SQLite adapter with CRDT-ready schema.
//!
//! Local-first storage. Every record has an HLC clock and origin_node
//! for eventual CRDT merge when nodes connect.

mod sqlite;

pub use sqlite::SqliteStore;
