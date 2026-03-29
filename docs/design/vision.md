# Rashk Vision

## The Problem

Every business tool — Supabase, n8n, Plane, Ghost, Chatwoot, Invoice Ninja, Cal.com — is a separate deployment, separate auth, separate data silo, separate UI. A solo builder runs 10-15 of these glued together with webhooks.

The current solutions:
- **SaaS sprawl** — pay for 20 subscriptions, data lives everywhere, privacy gone
- **Self-host everything** — DevOps nightmare, 15 Docker containers, still siloed

Neither works.

## The Insight

Five convergences make a new approach possible:

1. **Rust** — single binary, safe, compiles to native AND WASM
2. **SvelteKit** — compiled UI, works in any browser on any machine
3. **WebAssembly** — sandboxed, portable, near-native speed, runs everywhere
4. **AI agents** — the universal integration layer (no more webhooks)
5. **Web3 / Solana** — decentralized identity, trust, and payments

## What Rashk Is

A **decentralized capability runtime** where:

- Capabilities (CRM, invoicing, CMS, etc.) are WASM modules installed on demand
- AI agents orchestrate across modules — one command crosses what used to be 4 apps
- Mesh networking connects machines P2P — zero config, encrypted
- Identity is cryptographic — you own it, not a platform
- Data is local-first — yours by default, synced when you choose
- Trust is verifiable — on-chain reputation, signed modules

## The Scale Spectrum

```
SINGLE USER          TEAM              COMMUNITY           ENTERPRISE
────────────         ────────          ──────────           ──────────
1 binary             N binaries        Module registry      Managed fleet
Local SQLite         CRDT mesh sync    Published .wasm      Federated mesh
Local LLM            Shared agents     Agent marketplace    Private + shared
Local keypair        Invite links      DID + reputation     SSO + DID hybrid
No payments          Split costs       Solana micropay      Contracts + tokens
Your machine         Your machines     Any machine          Any infrastructure
```

Same binary at every stage. The composition root wires different adapters.

## Port Traits: The Universal Abstraction

The hexagonal architecture means:

```
Engine code → Port trait → Adapter (local / WASM / mesh / cloud / chain)
```

A port doesn't care if the adapter is:
- Local SQLite on disk
- A WASM module in a sandbox
- Another node on the mesh
- A cloud service
- A Solana smart contract

Engine code never changes. Only adapter wiring.

## The Open Source Ecosystem

We don't rewrite Supabase or n8n. We:
1. Define the module interface (WIT)
2. Build the runtime (wasmtime host)
3. Wrap existing open-source core logic as WASM modules
4. Let AI be the glue
5. Ship the registry

## What Makes This Unkillable

- Runs without internet (local-first, local LLM)
- Runs without us (open source, forkable registry)
- Runs without cloud (P2P mesh)
- Runs without tokens (web3 is optional)
- Gets better with network but doesn't require it

Every layer is optional except the runtime. A user can stop at "single binary on my laptop" and it's already useful.

## Relationship to RUSVEL

RUSVEL proved the architecture: hexagonal ports & adapters, departments as modules, AI agent orchestration. Rashk generalizes it:

| RUSVEL | Rashk |
|---|---|
| 50 compiled-in crates | Core runtime + WASM modules |
| DepartmentApp trait | WASM Component Model |
| 12 fixed departments | Any module, on demand |
| Single machine | Mesh of machines |
| Local identity | Cryptographic + on-chain identity |
| Free | Optional marketplace economy |

RUSVEL is a product. Rashk is the platform that products like RUSVEL run on.
