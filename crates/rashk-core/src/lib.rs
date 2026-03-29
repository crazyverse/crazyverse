//! rashk-core: The constitution.
//!
//! Every port trait here is the universal abstraction that works at every scale:
//! local SQLite, WASM sandbox, mesh peer, cloud service, or on-chain contract.
//! Engine code depends ONLY on these traits. Adapters implement them.

pub mod ports;
pub mod types;
pub mod error;
pub mod module;
pub mod capability;

pub use error::RashkError;
pub use types::*;
pub use module::{ModuleManifest, ModuleId};
pub use capability::{Capability, CapabilityGrant};
