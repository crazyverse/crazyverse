use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique module identifier — publisher/name convention.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModuleId(pub String);

impl ModuleId {
    pub fn new(publisher: &str, name: &str) -> Self {
        Self(format!("{publisher}/{name}"))
    }

    pub fn publisher(&self) -> &str {
        self.0.split('/').next().unwrap_or(&self.0)
    }

    pub fn name(&self) -> &str {
        self.0.split('/').nth(1).unwrap_or(&self.0)
    }
}

impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Module manifest — what a module IS, what it CAN DO, what it NEEDS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub id: ModuleId,
    pub version: String,
    pub description: String,

    /// What kind of module: "department", "tool", "adapter", "ui-fragment"
    pub kind: ModuleKind,

    /// Capabilities this module REQUIRES (filesystem, network, store, etc.)
    pub required_capabilities: Vec<String>,

    /// Ports this module EXPORTS (implements).
    pub exports: Vec<String>,

    /// Ports this module IMPORTS (needs injected).
    pub imports: Vec<String>,

    /// UI entry point (if module has a frontend component).
    pub ui_entry: Option<String>,

    /// Publisher's public key for signature verification.
    pub publisher_key: Option<String>,

    /// Signature over the manifest + wasm hash.
    pub signature: Option<String>,

    /// Size of the .wasm binary in bytes.
    pub wasm_size: Option<u64>,

    /// SHA-256 hash of the .wasm binary.
    pub wasm_hash: Option<String>,

    pub published_at: Option<DateTime<Utc>>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    /// Full department (like rusvel's dept-* crates)
    Department,
    /// Single tool that agents can use
    Tool,
    /// Port adapter (store, llm, payment, etc.)
    Adapter,
    /// UI component / page fragment
    UiFragment,
    /// Workflow / automation
    Workflow,
}
