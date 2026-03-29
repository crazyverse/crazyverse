# P11: DID Methods in Practice

> Research prompt: Which Decentralized Identifier (DID) methods have real adoption as of 2026?

## Key Findings

### DID Methods Evaluated

**did:key** — Widely implemented, simplest method. Used for ephemeral/peer-to-peer identity (SSH-style). Ed25519 native support. No registry, no on-chain footprint. Good for local agents and wallets. The "just works" option for day-one identity.

**did:web** — Broadest practical adoption of any DID method. Any domain can host a `did.json` file. Ed25519 supported. **did:webvh** (verifiable history) adds tamper-evident history without requiring a ledger. Good for organizations and public-facing identity.

**did:sol** — Rust crate exists (`did-sol` on crates.io, maintained by SpruceID). Ed25519 native (Solana keys are Ed25519). Limited adoption — mostly ecosystem projects and experiments. Useful when Solana-native method is specifically needed.

**did:ion** — Effectively frozen/legacy. Microsoft shifted focus to Entra Verified ID (centralized). Poor choice for new deployments. Avoid.

**did:pkh** — Bridges Web3 wallets to the DID world. Wallet address = DID (no separate registration). Supported by SpruceID Rust tooling. Significant adoption where Web3 meets Verifiable Credentials / SSI. Particularly relevant for Solana where Ed25519 wallet address can directly become a DID.

### Verifiable Credentials in Rust

**SpruceID stack** — Primary Rust ecosystem for DIDs and VCs:
- `ssi` — Core DID resolution and VC data models
- `ssi-vc` — Verifiable Credential issuance and verification
- `DIDKit` — Higher-level toolkit wrapping ssi
- Supports Ed25519 signing, did:key, did:web, did:pkh, did:sol

**Other crates:**
- `identity_iota` — IOTA-specific, less relevant for Solana
- `openid4vc` — OpenID for Verifiable Credentials, bridges traditional auth with VCs

## RASHK Recommendation

### DID Method Strategy (Progressive)

| Method | Use Case | When |
|--------|----------|------|
| `did:key` | Local identity, ephemeral agents, peer-to-peer | Day 1 (simplest, no infra needed) |
| `did:pkh` | Wallet-anchored identity (Solana Ed25519 address as DID) | Day 1 (wallet = DID, no registration) |
| `did:web` | DNS-backed public identity for organizations/services | Phase 4+ (requires domain hosting) |
| `did:sol` | Solana-native on-chain DID when needed | Phase 5+ (when on-chain resolution matters) |

### Implementation Stack

- **SpruceID Rust stack** (`ssi` + `DIDKit`) for VC issuance and verification
- Ed25519 as the universal key type (works across all chosen DID methods and Solana)
- Start simple: `did:key` for local, `did:pkh` for wallet-linked
- Add methods as needed without changing the `IdentityPort` trait

## Decision Impact

Informs **D4 (DID/identity method)**:
- Start with `did:key` (simplest, works day 1) + `did:pkh` (wallet = DID, zero registration)
- Progress to `did:web` and `did:sol` as the system matures
- SpruceID Rust stack for all Verifiable Credential operations
- Ed25519 key type unifies Solana wallet keys and DID signing keys
