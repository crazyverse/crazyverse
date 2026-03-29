use async_trait::async_trait;
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};
use rashk_core::error::RashkError;
use rashk_core::ports::{Identity, IdentityPort};
use rashk_core::types::NodeId;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;

/// Local identity backed by an Ed25519 keypair.
pub struct LocalIdentity {
    node_id: NodeId,
    signing_key: SigningKey,
    known_keys: Mutex<HashMap<String, VerifyingKey>>,
}

impl LocalIdentity {
    /// Generate a new random identity.
    pub fn generate() -> Self {
        let mut secret = [0u8; 32];
        rand::RngCore::fill_bytes(&mut rand::rngs::OsRng, &mut secret);
        let signing_key = SigningKey::from_bytes(&secret);
        let node_id = NodeId::new();
        Self {
            node_id,
            signing_key,
            known_keys: Mutex::new(HashMap::new()),
        }
    }

    /// Create from an existing keypair bytes (32 bytes).
    pub fn from_bytes(node_id: NodeId, key_bytes: &[u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(key_bytes);
        Self {
            node_id,
            signing_key,
            known_keys: Mutex::new(HashMap::new()),
        }
    }

    /// Load identity from disk, or generate and save a new one.
    pub fn load_or_generate(path: &Path) -> Result<Self, RashkError> {
        if path.exists() {
            Self::load(path)
        } else {
            let identity = Self::generate();
            identity.save(path)?;
            Ok(identity)
        }
    }

    /// Save identity (node_id + secret key) to a file.
    pub fn save(&self, path: &Path) -> Result<(), RashkError> {
        // Format: node_id (utf8, newline-terminated) + 32 bytes secret key
        let mut data = self.node_id.0.as_bytes().to_vec();
        data.push(b'\n');
        data.extend_from_slice(&self.signing_key.to_bytes());
        std::fs::write(path, &data)
            .map_err(|e| RashkError::Identity(format!("save identity: {e}")))?;
        // Restrict file permissions on unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
                .map_err(|e| RashkError::Identity(format!("set permissions: {e}")))?;
        }
        Ok(())
    }

    /// Load identity from a file.
    pub fn load(path: &Path) -> Result<Self, RashkError> {
        let data = std::fs::read(path)
            .map_err(|e| RashkError::Identity(format!("read identity: {e}")))?;
        let newline_pos = data.iter().position(|&b| b == b'\n')
            .ok_or_else(|| RashkError::Identity("invalid identity file format".into()))?;
        let node_id_str = std::str::from_utf8(&data[..newline_pos])
            .map_err(|e| RashkError::Identity(format!("invalid node_id: {e}")))?;
        let key_bytes = &data[newline_pos + 1..];
        let key_array: [u8; 32] = key_bytes.try_into()
            .map_err(|_| RashkError::Identity("invalid key length (expected 32 bytes)".into()))?;
        Ok(Self::from_bytes(NodeId::from_str(node_id_str), &key_array))
    }

    /// Register a known peer's public key for verification.
    pub fn register_peer(&self, node_id: &NodeId, public_key: &[u8]) -> Result<(), RashkError> {
        let verifying_key = VerifyingKey::from_bytes(
            public_key
                .try_into()
                .map_err(|_| RashkError::Identity("invalid public key length".into()))?,
        )
        .map_err(|e| RashkError::Identity(e.to_string()))?;

        self.known_keys
            .lock()
            .unwrap()
            .insert(node_id.0.clone(), verifying_key);
        Ok(())
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.signing_key.verifying_key().to_bytes().to_vec()
    }
}

#[async_trait]
impl IdentityPort for LocalIdentity {
    async fn local_identity(&self) -> Result<Identity, RashkError> {
        Ok(Identity {
            node_id: self.node_id.clone(),
            public_key: self.public_key_bytes(),
            did: None, // Phase 5: DID resolution
            name: None,
            metadata: serde_json::json!({}),
        })
    }

    async fn sign(&self, data: &[u8]) -> Result<Vec<u8>, RashkError> {
        let signature = self.signing_key.sign(data);
        Ok(signature.to_bytes().to_vec())
    }

    async fn verify(&self, peer: &NodeId, data: &[u8], signature: &[u8]) -> Result<bool, RashkError> {
        let keys = self.known_keys.lock().unwrap();
        let verifying_key = keys
            .get(&peer.0)
            .ok_or_else(|| RashkError::Identity(format!("unknown peer: {peer}")))?;

        let sig = ed25519_dalek::Signature::from_bytes(
            signature
                .try_into()
                .map_err(|_| RashkError::Identity("invalid signature length".into()))?,
        );

        Ok(verifying_key.verify(data, &sig).is_ok())
    }

    async fn resolve_did(&self, _did: &str) -> Result<Option<Identity>, RashkError> {
        // Phase 5: DID resolution via Solana or other DID methods
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_and_sign() {
        let id = LocalIdentity::generate();
        let identity = id.local_identity().await.unwrap();
        assert!(!identity.public_key.is_empty());

        let data = b"hello rashk";
        let sig = id.sign(data).await.unwrap();
        assert_eq!(sig.len(), 64); // Ed25519 signature
    }

    #[tokio::test]
    async fn test_save_and_load() {
        let dir = std::env::temp_dir().join("rashk_identity_test");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_identity.key");

        let original = LocalIdentity::generate();
        let original_id = original.local_identity().await.unwrap();
        original.save(&path).unwrap();

        let loaded = LocalIdentity::load(&path).unwrap();
        let loaded_id = loaded.local_identity().await.unwrap();

        assert_eq!(original_id.node_id, loaded_id.node_id);
        assert_eq!(original_id.public_key, loaded_id.public_key);

        // Loaded identity can sign and original can verify
        let data = b"round-trip test";
        let sig = loaded.sign(data).await.unwrap();
        original.register_peer(&loaded_id.node_id, &loaded_id.public_key).unwrap();
        let valid = original.verify(&loaded_id.node_id, data, &sig).await.unwrap();
        assert!(valid);

        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_load_or_generate() {
        let dir = std::env::temp_dir().join("rashk_identity_test_log");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test_identity2.key");
        let _ = std::fs::remove_file(&path);

        // First call generates
        let id1 = LocalIdentity::load_or_generate(&path).unwrap();
        let id1_info = id1.local_identity().await.unwrap();
        assert!(path.exists());

        // Second call loads the same identity
        let id2 = LocalIdentity::load_or_generate(&path).unwrap();
        let id2_info = id2.local_identity().await.unwrap();
        assert_eq!(id1_info.node_id, id2_info.node_id);
        assert_eq!(id1_info.public_key, id2_info.public_key);

        let _ = std::fs::remove_file(&path);
    }

    #[tokio::test]
    async fn test_sign_and_verify() {
        let alice = LocalIdentity::generate();
        let bob = LocalIdentity::generate();

        let alice_id = alice.local_identity().await.unwrap();
        bob.register_peer(&alice_id.node_id, &alice_id.public_key).unwrap();

        let data = b"message from alice";
        let sig = alice.sign(data).await.unwrap();

        let valid = bob.verify(&alice_id.node_id, data, &sig).await.unwrap();
        assert!(valid);

        let tampered = bob.verify(&alice_id.node_id, b"tampered", &sig).await.unwrap();
        assert!(!tampered);
    }
}
