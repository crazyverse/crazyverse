use async_trait::async_trait;
use rashk_core::error::RashkError;
use rashk_core::ports::MeshPort;
use rashk_core::types::{NodeId, Peer};
use std::collections::HashMap;
use std::sync::Mutex;

/// QUIC-based mesh networking.
/// Phase 1: local tracking. Phase 2: real QUIC connections.
pub struct QuicMesh {
    local_node: NodeId,
    peers: Mutex<HashMap<String, Peer>>,
}

impl QuicMesh {
    pub fn new(node_id: NodeId) -> Self {
        Self {
            local_node: node_id,
            peers: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl MeshPort for QuicMesh {
    async fn local_node(&self) -> Result<NodeId, RashkError> {
        Ok(self.local_node.clone())
    }

    async fn discover(&self) -> Result<Vec<Peer>, RashkError> {
        // Phase 1: return known peers. Phase 2: mDNS + QUIC discovery.
        let peers = self.peers.lock().unwrap();
        Ok(peers.values().cloned().collect())
    }

    async fn connect(&self, node_id: &NodeId) -> Result<(), RashkError> {
        tracing::info!("connecting to peer: {node_id}");
        // Phase 2: establish QUIC connection
        Ok(())
    }

    async fn disconnect(&self, node_id: &NodeId) -> Result<(), RashkError> {
        self.peers.lock().unwrap().remove(&node_id.0);
        Ok(())
    }

    async fn send(
        &self,
        _to: &NodeId,
        _channel: &str,
        _payload: bytes::Bytes,
    ) -> Result<(), RashkError> {
        // Phase 2: send via QUIC stream
        Err(RashkError::Mesh("send not yet wired".into()))
    }

    async fn peers(&self) -> Result<Vec<Peer>, RashkError> {
        let peers = self.peers.lock().unwrap();
        Ok(peers.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mesh_local_node() {
        let mesh = QuicMesh::new(NodeId::from_str("test-node-1"));
        let id = mesh.local_node().await.unwrap();
        assert_eq!(id.0, "test-node-1");
    }

    #[tokio::test]
    async fn test_mesh_discover_empty() {
        let mesh = QuicMesh::new(NodeId::new());
        let peers = mesh.discover().await.unwrap();
        assert!(peers.is_empty());
    }
}
