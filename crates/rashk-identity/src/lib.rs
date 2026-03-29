//! rashk-identity: Cryptographic identity and trust.
//!
//! Local keypair generation, signing, verification.
//! Future: DID resolution, Solana on-chain identity, verifiable credentials.

mod keypair;

pub use keypair::LocalIdentity;
