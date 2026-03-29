use thiserror::Error;

#[derive(Error, Debug)]
pub enum RashkError {
    #[error("store: {0}")]
    Store(String),

    #[error("module: {0}")]
    Module(String),

    #[error("mesh: {0}")]
    Mesh(String),

    #[error("identity: {0}")]
    Identity(String),

    #[error("capability denied: {0}")]
    CapabilityDenied(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

impl From<serde_json::Error> for RashkError {
    fn from(e: serde_json::Error) -> Self {
        Self::Other(e.to_string())
    }
}
