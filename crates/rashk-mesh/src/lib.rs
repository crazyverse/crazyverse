//! rashk-mesh: P2P mesh networking.
//!
//! Encrypted, zero-config node discovery and communication.
//! Enables remote port proxying — a Store on node A can be
//! transparently accessed from node B.

mod quic;

pub use quic::QuicMesh;
