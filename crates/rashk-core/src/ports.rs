//! Universal port traits.
//!
//! Every trait here can be implemented by:
//! - A local adapter (SQLite, filesystem, in-memory)
//! - A WASM guest module (sandboxed)
//! - A mesh proxy (delegates to a remote node)
//! - A cloud service (managed API)
//! - A blockchain adapter (on-chain state)
//!
//! Engine code uses ONLY these traits. The composition root wires adapters.

use async_trait::async_trait;

use crate::error::RashkError;
use crate::module::{ModuleId, ModuleManifest};
use crate::types::*;
use crate::capability::{Capability, CapabilityGrant};

type Result<T> = std::result::Result<T, RashkError>;

// ─── Storage ───────────────────────────────────────────────────

/// Universal key-value + document store.
/// The adapter decides WHERE data lives (local, synced, remote, on-chain).
#[async_trait]
pub trait Store: Send + Sync {
    async fn get(&self, id: &RecordId) -> Result<Option<Record>>;
    async fn put(&self, record: &Record) -> Result<()>;
    async fn delete(&self, id: &RecordId) -> Result<()>;
    async fn query(&self, query: &Query) -> Result<Vec<Record>>;
    async fn count(&self, query: &Query) -> Result<u64>;
}

/// Blob storage — files, media, WASM module binaries.
#[async_trait]
pub trait BlobStore: Send + Sync {
    async fn put_blob(&self, key: &str, blob: &Blob) -> Result<()>;
    async fn get_blob(&self, key: &str) -> Result<Option<Blob>>;
    async fn delete_blob(&self, key: &str) -> Result<()>;
    async fn list_blobs(&self, prefix: &str) -> Result<Vec<String>>;
}

/// Full-text search.
#[async_trait]
pub trait SearchIndex: Send + Sync {
    async fn index(&self, id: &RecordId, kind: &str, text: &str) -> Result<()>;
    async fn search(&self, query: &str, limit: u32) -> Result<Vec<RecordId>>;
    async fn remove(&self, id: &RecordId) -> Result<()>;
}

/// Vector store for embeddings / semantic search.
#[async_trait]
pub trait VectorStore: Send + Sync {
    async fn upsert(&self, id: &RecordId, embedding: &[f32], metadata: serde_json::Value) -> Result<()>;
    async fn search_similar(&self, embedding: &[f32], limit: u32) -> Result<Vec<(RecordId, f32)>>;
    async fn delete(&self, id: &RecordId) -> Result<()>;
}

// ─── Sync ──────────────────────────────────────────────────────

/// CRDT-based sync between nodes. Local-first, merge on connect.
#[async_trait]
pub trait SyncPort: Send + Sync {
    /// Get changes since the given HLC for a peer.
    async fn changes_since(&self, hlc: u64, peer: &NodeId) -> Result<Vec<Record>>;
    /// Merge incoming changes from a peer.
    async fn merge(&self, records: Vec<Record>, from: &NodeId) -> Result<u64>;
    /// Current HLC clock value.
    async fn current_hlc(&self) -> Result<u64>;
}

// ─── Events ────────────────────────────────────────────────────

/// Event bus — publish/subscribe across modules and nodes.
#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: Event) -> Result<()>;
    async fn subscribe(&self, pattern: &str) -> Result<Box<dyn EventStream>>;
    async fn replay(&self, kind: &str, since: chrono::DateTime<chrono::Utc>) -> Result<Vec<Event>>;
}

/// Stream of events (async iterator pattern).
#[async_trait]
pub trait EventStream: Send + Sync {
    async fn next(&self) -> Result<Option<Event>>;
}

// ─── Modules / WASM ────────────────────────────────────────────

/// Module registry — discover, install, run capability modules.
#[async_trait]
pub trait ModuleRegistry: Send + Sync {
    /// List installed modules.
    async fn list_installed(&self) -> Result<Vec<ModuleManifest>>;
    /// Install a module from bytes (local .wasm file or fetched from registry).
    async fn install(&self, manifest: &ModuleManifest, wasm_bytes: &[u8]) -> Result<ModuleId>;
    /// Uninstall a module.
    async fn uninstall(&self, id: &ModuleId) -> Result<()>;
    /// Get module manifest.
    async fn get_manifest(&self, id: &ModuleId) -> Result<Option<ModuleManifest>>;
}

/// Remote module registry — fetch from cloud / community registry.
#[async_trait]
pub trait ModuleSource: Send + Sync {
    /// Search available modules.
    async fn search(&self, query: &str) -> Result<Vec<ModuleManifest>>;
    /// Download module bytes.
    async fn fetch(&self, id: &ModuleId, version: &str) -> Result<Vec<u8>>;
    /// Publish a module.
    async fn publish(&self, manifest: &ModuleManifest, wasm_bytes: &[u8]) -> Result<()>;
}

/// Module runtime — execute WASM modules with capability grants.
#[async_trait]
pub trait ModuleRuntime: Send + Sync {
    /// Load and instantiate a module.
    async fn load(&self, id: &ModuleId) -> Result<()>;
    /// Call a function on a loaded module.
    async fn call(
        &self,
        id: &ModuleId,
        function: &str,
        input: serde_json::Value,
    ) -> Result<serde_json::Value>;
    /// Unload a module.
    async fn unload(&self, id: &ModuleId) -> Result<()>;
    /// List loaded modules.
    async fn loaded(&self) -> Result<Vec<ModuleId>>;
}

// ─── Mesh Networking ───────────────────────────────────────────

/// Mesh network — discover and connect to peers.
#[async_trait]
pub trait MeshPort: Send + Sync {
    /// This node's identity.
    async fn local_node(&self) -> Result<NodeId>;
    /// Discover peers on the network.
    async fn discover(&self) -> Result<Vec<Peer>>;
    /// Connect to a specific peer.
    async fn connect(&self, node_id: &NodeId) -> Result<()>;
    /// Disconnect from a peer.
    async fn disconnect(&self, node_id: &NodeId) -> Result<()>;
    /// Send a message to a peer.
    async fn send(&self, to: &NodeId, channel: &str, payload: bytes::Bytes) -> Result<()>;
    /// List connected peers.
    async fn peers(&self) -> Result<Vec<Peer>>;
}

/// Receive messages from mesh peers.
#[async_trait]
pub trait MeshReceiver: Send + Sync {
    async fn recv(&self, channel: &str) -> Result<Option<(NodeId, bytes::Bytes)>>;
}

// ─── Identity & Trust ──────────────────────────────────────────

/// Identity management — create, verify, present.
#[async_trait]
pub trait IdentityPort: Send + Sync {
    /// Get or create the local identity.
    async fn local_identity(&self) -> Result<Identity>;
    /// Sign arbitrary data.
    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>>;
    /// Verify a signature from a peer.
    async fn verify(&self, peer: &NodeId, data: &[u8], signature: &[u8]) -> Result<bool>;
    /// Resolve a DID to an identity.
    async fn resolve_did(&self, did: &str) -> Result<Option<Identity>>;
}

/// Local identity — keypair + optional DID + metadata.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Identity {
    pub node_id: NodeId,
    pub public_key: Vec<u8>,
    pub did: Option<String>,
    pub name: Option<String>,
    pub metadata: serde_json::Value,
}

/// Trust and reputation — verify modules, rate peers.
#[async_trait]
pub trait TrustPort: Send + Sync {
    /// Verify a module's signature against its publisher.
    async fn verify_module(&self, manifest: &ModuleManifest, signature: &[u8]) -> Result<bool>;
    /// Get reputation score for a node or module publisher.
    async fn reputation(&self, node_id: &NodeId) -> Result<f64>;
}

// ─── Capabilities / Permissions ────────────────────────────────

/// Capability manager — grant, revoke, check permissions for modules.
#[async_trait]
pub trait CapabilityPort: Send + Sync {
    /// Grant a capability to a module.
    async fn grant(&self, module: &ModuleId, cap: CapabilityGrant) -> Result<()>;
    /// Revoke a capability.
    async fn revoke(&self, module: &ModuleId, cap: &Capability) -> Result<()>;
    /// Check if a module has a capability.
    async fn check(&self, module: &ModuleId, cap: &Capability) -> Result<bool>;
    /// List all grants for a module.
    async fn grants(&self, module: &ModuleId) -> Result<Vec<CapabilityGrant>>;
}

// ─── AI / Agent ────────────────────────────────────────────────

/// LLM port — same as RUSVEL, works with any provider.
#[async_trait]
pub trait LlmPort: Send + Sync {
    async fn complete(&self, prompt: &str, system: Option<&str>) -> Result<String>;
    async fn complete_json(
        &self,
        prompt: &str,
        system: Option<&str>,
    ) -> Result<serde_json::Value>;
}

/// Agent port — tool-using AI agent.
#[async_trait]
pub trait AgentPort: Send + Sync {
    async fn run(
        &self,
        prompt: &str,
        tools: &[ToolDef],
        context: serde_json::Value,
    ) -> Result<AgentResult>;
}

/// Tool definition for agents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub schema: serde_json::Value,
}

/// Result of an agent run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResult {
    pub output: String,
    pub tool_calls: Vec<ToolCall>,
    pub metadata: serde_json::Value,
}

/// A tool call made by an agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub tool: String,
    pub input: serde_json::Value,
    pub output: serde_json::Value,
}

// ─── Payments (Web3) ───────────────────────────────────────────

/// Payment port — optional, for marketplace and agent-to-agent transactions.
#[async_trait]
pub trait PaymentPort: Send + Sync {
    /// Get balance.
    async fn balance(&self) -> Result<PaymentBalance>;
    /// Send payment to a node/address.
    async fn send_payment(&self, to: &str, amount: u64, memo: Option<&str>) -> Result<String>;
    /// Verify a payment was made.
    async fn verify_payment(&self, tx_id: &str) -> Result<PaymentStatus>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentBalance {
    pub amount: u64,
    pub currency: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentStatus {
    Pending,
    Confirmed { block: u64 },
    Failed { reason: String },
}

use serde::{Serialize, Deserialize};
