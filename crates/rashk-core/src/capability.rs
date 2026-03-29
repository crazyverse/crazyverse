use serde::{Deserialize, Serialize};

/// A capability that can be granted to a WASM module.
/// Fine-grained: a module can only do what it's explicitly allowed to do.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    /// Read from the store (optionally restricted to a record kind).
    StoreRead(Option<String>),
    /// Write to the store (optionally restricted to a record kind).
    StoreWrite(Option<String>),
    /// Read blobs.
    BlobRead,
    /// Write blobs.
    BlobWrite,
    /// Full-text search.
    Search,
    /// Vector/semantic search.
    VectorSearch,
    /// Publish events.
    EventPublish,
    /// Subscribe to events (optionally restricted to a pattern).
    EventSubscribe(Option<String>),
    /// Make outbound HTTP requests (optionally restricted to a domain).
    NetworkOutbound(Option<String>),
    /// Access the filesystem (scoped to a path).
    Filesystem(String),
    /// Use LLM/Agent capabilities.
    Ai,
    /// Send mesh messages.
    MeshSend,
    /// Make payments (with optional max amount).
    Payment(Option<u64>),
    /// Custom capability defined by a module.
    Custom(String),
}

/// A granted capability with optional constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityGrant {
    pub capability: Capability,
    /// Who granted it (user, system, or another module).
    pub granted_by: String,
    /// Optional expiry.
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Optional rate limit (calls per minute).
    pub rate_limit: Option<u32>,
}
