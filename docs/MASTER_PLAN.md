# RASHK — Master Plan

> From RUSVEL (single-user virtual agency) to RASHK (decentralized operating system for work).
> Same hexagonal core. New layers: WASM modules, mesh networking, CRDT sync, Web3 identity, token economy.

**Status:** Phase 0 — Design & Research
**Started:** 2026-03-27
**Author:** Mehdi Baneshi

---

## 1. The Evolution

### What RUSVEL Is Today

A **single Rust binary** (50 crates, ~52K LOC) that runs a multi-department virtual agency:
- 19 port traits (14 Port + 5 Store sub-traits) in `rusvel-core`
- 13 departments via `DepartmentApp` trait (5 wired engines + 8 stubs)
- SvelteKit frontend, CLI, REPL, TUI, MCP surfaces
- SQLite local-first, 4 LLM providers, LanceDB vectors, agent runtime with tool loop
- Hexagonal architecture — engines depend ONLY on port traits

### What RASHK Becomes

A **decentralized capability runtime** where:
- The binary is a **host**, not a monolith — departments become installable WASM modules
- Machines **discover each other** via encrypted P2P mesh (Tailscale model)
- Data is **local-first with CRDT sync** — works offline, merges on connect
- Identity is **self-sovereign** (DID/Web3) — you own it, not a platform
- AI agents are **the kernel** — they orchestrate across modules, nodes, and chains
- A **token economy** makes it self-sustaining — module marketplace, micropayments, governance

### Why Ports Make This Possible

The same port trait works at every scale. The engine code never changes — only the adapter wiring:

| Scale | Store adapter | Mesh adapter | Identity adapter |
|-------|--------------|-------------|-----------------|
| Solo (day 1) | `SqliteStore` | None | Local keypair |
| Multi-device | `CrdtSqliteStore` | mDNS/local | Local keypair |
| Team (5 people) | `CrdtSyncStore` | WireGuard mesh | Invite links |
| Community | `DistributedStore` | Public mesh | DID + reputation |
| Enterprise | `ShardedStore` | Federated mesh | SSO + DID hybrid |
| On-chain | `SolanaAnchorStore` | N/A | Solana DID |
| Browser | `WasmSqliteStore` | WebRTC | Browser wallet |

---

## 2. Architecture Layers

```
Layer 5: AI Orchestration ─── agents delegate across nodes, negotiate on-chain
Layer 4: Identity & Trust ─── DID, verifiable credentials, module signing, reputation
Layer 3: Mesh Networking  ─── P2P discovery, encrypted tunnels, remote port proxying
Layer 2: WASM Modules     ─── on-demand install, sandboxed, hot-swappable departments
Layer 1: Runtime          ─── single binary, Rust core, SvelteKit UI, port traits
```

Every layer is optional except Layer 1. A user can stop at "single binary on my laptop" and it's already useful.

---

## 3. Port Trait Audit

### RUSVEL → RASHK Port Mapping

| RUSVEL Port (19) | RASHK Equivalent | Change | Notes |
|-----------------|-----------------|--------|-------|
| `LlmPort` | `LlmPort` | Simplified | Drop batch API for now, add back later |
| `AgentPort` | `AgentPort` | Simplified | Prompt + tools + context in, result out |
| `ToolPort` | (merged into AgentPort) | Removed | Tools are module-level, not a port |
| `EventPort` | `EventBus` | Evolved | Add pub/sub pattern + replay + cross-node |
| `StoragePort` (5 sub-stores) | `Store` + `BlobStore` | Simplified | Generic record store + blob store |
| `MemoryPort` | (via Store + VectorStore) | Merged | Memory is just Store + search |
| `JobPort` | (via EventBus + modules) | Merged | Jobs become events that modules consume |
| `SessionPort` | (via Store) | Merged | Sessions are records in Store |
| `AuthPort` | (via IdentityPort) | Merged | Auth = identity + capability grants |
| `ConfigPort` | (via Store) | Merged | Config is records in Store |
| `EmbeddingPort` | (via LlmPort or module) | Merged | Embedding is an LLM capability |
| `VectorStorePort` | `VectorStore` | Kept | Semantic search is fundamental |
| `DeployPort` | (module) | Removed | Deploy becomes a WASM module |
| `TerminalPort` | (module) | Removed | Terminal becomes a WASM module |
| `BrowserPort` | (module) | Removed | Browser becomes a WASM module |
| — | `SearchIndex` | **NEW** | Full-text search as first-class port |
| — | `SyncPort` | **NEW** | CRDT-based sync between nodes |
| — | `ModuleRegistry` | **NEW** | Install/uninstall WASM modules locally |
| — | `ModuleSource` | **NEW** | Fetch modules from cloud registry |
| — | `ModuleRuntime` | **NEW** | Load, call, unload WASM modules |
| — | `MeshPort` | **NEW** | P2P discovery, connect, send/receive |
| — | `MeshReceiver` | **NEW** | Receive messages from mesh peers |
| — | `IdentityPort` | **NEW** | Local keypair, DID, signing, verification |
| — | `TrustPort` | **NEW** | Module verification, reputation scores |
| — | `CapabilityPort` | **NEW** | Grant/revoke/check WASI permissions |
| — | `PaymentPort` | **NEW** | Balance, send, verify (Solana) |

### Design Principle

RASHK has **fewer, more general ports** than RUSVEL. Specialized concerns (terminal, browser, deploy) move from core ports to installable WASM modules. New ports cover the distributed/decentralized layers.

**RASHK core ports (16):**

| # | Port | Layer | Purpose |
|---|------|-------|---------|
| 1 | `Store` | 1 | Key-value + document records |
| 2 | `BlobStore` | 1 | Files, media, WASM binaries |
| 3 | `SearchIndex` | 1 | Full-text search |
| 4 | `VectorStore` | 1 | Embedding similarity search |
| 5 | `EventBus` | 1 | Pub/sub events + replay |
| 6 | `LlmPort` | 1 | LLM completion + embeddings |
| 7 | `AgentPort` | 1 | Tool-using AI agent |
| 8 | `SyncPort` | 3 | CRDT merge between nodes |
| 9 | `ModuleRegistry` | 2 | Local module management |
| 10 | `ModuleSource` | 2 | Remote registry (fetch/publish) |
| 11 | `ModuleRuntime` | 2 | WASM sandbox execution |
| 12 | `MeshPort` | 3 | P2P discovery + messaging |
| 13 | `MeshReceiver` | 3 | Receive from mesh peers |
| 14 | `IdentityPort` | 4 | Keypair, DID, signing |
| 15 | `TrustPort` | 4 | Module verification, reputation |
| 16 | `CapabilityPort` | 2 | WASI permission grants |
| 17 | `PaymentPort` | 4 | Solana transactions (optional) |

---

## 4. Research Questions

Before building, we need answers. Each question maps to a phase and has a suggested research tool.

### R1: WASM Runtime (Phase 1)

| # | Question | Research with |
|---|----------|--------------|
| R1.1 | **wasmtime vs wasmer vs extism** — which is the right host for Rust? Extism wraps wasmtime but adds plugin ergonomics. Is the abstraction worth it? | Perplexity |
| R1.2 | **WASI Preview 2 + Component Model** — what's the actual state in March 2026? Can we define WIT interfaces for our ports? | Perplexity |
| R1.3 | **wit-bindgen** — can we auto-generate Rust host↔guest bindings from our port traits? | Gemini |
| R1.4 | **Performance overhead** — what's the realistic cost of calling a WASM function vs native Rust? Per-call latency, memory overhead? | Perplexity |
| R1.5 | **Existing WASM modules** — what already compiles to WASM? SQLite (sql.js, cr-sqlite), FFmpeg, Postgres (PGlite)? What's production-ready? | Perplexity |
| R1.6 | **Module hot-reload** — can we unload and reload a WASM module without restarting the host? | Claude |

### R2: CRDT / Data Sync (Phase 2)

| # | Question | Research with |
|---|----------|--------------|
| R2.1 | **cr-sqlite vs Automerge vs Yjs** — which CRDT approach fits a Rust host? cr-sqlite keeps SQLite compatibility. Automerge is pure CRDT. | Perplexity |
| R2.2 | **cr-sqlite maturity** — is it production-ready? What conflict types does it handle? Schema migrations? | Perplexity |
| R2.3 | **HLC (Hybrid Logical Clocks)** — how do other local-first systems implement causal ordering across nodes? | Gemini |
| R2.4 | **Sync protocol design** — what does the sync message format look like? How much bandwidth for N nodes with M records? | Claude |

### R3: Mesh Networking (Phase 3)

| # | Question | Research with |
|---|----------|--------------|
| R3.1 | **Tailscale architecture** — how does their coordination server + WireGuard tunnel model work? Can we replicate the key parts? | Perplexity |
| R3.2 | **QUIC vs WireGuard vs libp2p** — for the transport layer. QUIC is simpler, WireGuard is proven, libp2p is feature-rich but complex. | Perplexity |
| R3.3 | **mDNS for local discovery** — what Rust crates exist? How reliable is mDNS in practice across OS? | Perplexity |
| R3.4 | **NAT traversal** — STUN/TURN/ICE for nodes behind NAT. What's the minimum viable approach? | Gemini |
| R3.5 | **quinn (QUIC in Rust)** — maturity, API ergonomics, TLS integration? | Perplexity |

### R4: Identity & Web3 (Phase 4)

| # | Question | Research with |
|---|----------|--------------|
| R4.1 | **DID methods** — did:key (simple), did:web (DNS-based), did:sol (Solana). Which to start with? | Perplexity |
| R4.2 | **Solana DID programs** — what exists on-chain for identity? Metaplex, DID registries? | Perplexity |
| R4.3 | **Ed25519 in Rust** — `ed25519-dalek` for local keypair, compatible with Solana's keypair format? | Claude |
| R4.4 | **Verifiable Credentials** — W3C VC spec, what Rust libraries exist? Can we issue/verify VCs locally? | Perplexity |
| R4.5 | **Module signing** — how should we sign .wasm files? Sigstore model? Simple Ed25519 detached sigs? | Gemini |

### R5: Token Economy (Phase 5)

| # | Question | Research with |
|---|----------|--------------|
| R5.1 | **Solana SPL tokens** — creating a token, transfer, program interactions from Rust | Perplexity |
| R5.2 | **Micropayment UX** — how to make sub-cent payments invisible to users? Pre-funded escrow? | Gemini |
| R5.3 | **Anchor framework** — for writing Solana programs in Rust. Maturity, testing story? | Perplexity |
| R5.4 | **Token governance** — lightweight on-chain voting for module standards? SPL Governance? Realms? | Perplexity |

### R6: Existing Open-Source to Wrap (Cross-cutting)

| # | Question | Research with |
|---|----------|--------------|
| R6.1 | **Which projects compile to WASM?** Test: Plane, n8n core, Supabase auth, PocketBase logic, Invoice Ninja | Perplexity |
| R6.2 | **Minimum viable module interface** — what does a "capability module" need to expose? UI fragments? Data schema? Tool registrations? | Claude |
| R6.3 | **Browser as deployment target** — can the same .wasm that runs in wasmtime also run in the browser via wasm-bindgen? Or do we need two builds? | Perplexity |

---

## 5. Implementation Phases

### Phase 0: Design & Research (NOW)
> Answer the research questions. Validate architecture. No code until we're confident.

- [ ] **P0.1** Complete port trait audit — finalize the 17 traits
- [ ] **P0.2** Research R1 (WASM runtime) — wasmtime vs extism, WASI P2, WIT
- [ ] **P0.3** Research R2 (CRDT) — cr-sqlite vs Automerge
- [ ] **P0.4** Research R3 (Mesh) — QUIC vs WireGuard, discovery
- [ ] **P0.5** Research R4 (Identity) — DID methods, Ed25519, module signing
- [ ] **P0.6** Research R6 (Modules) — what compiles to WASM, module interface design
- [ ] **P0.7** Write ADR for each major decision
- [ ] **P0.8** Define WIT interface for one port (Store) as proof of concept
- [ ] **P0.9** Validate: can a dept-finance equivalent work as both native Rust AND .wasm?

**Exit criteria:** We know the tech stack for each layer. ADRs written. One WIT proof-of-concept validates the approach.

---

### Phase 1: Runtime + WASM Module System (Weeks 1-4)
> The binary becomes a host. Departments become installable.

- [ ] **P1.1** Set up workspace: `rashk-core`, `rashk-store`, `rashk-wasm`, `rashk-app`
- [ ] **P1.2** Define 17 port traits in `rashk-core` (finalized from P0)
- [ ] **P1.3** Implement local adapters: SQLite Store, filesystem BlobStore, FTS5 SearchIndex
- [ ] **P1.4** Integrate wasmtime/extism as WASM host in `rashk-wasm`
- [ ] **P1.5** Define WIT interfaces for core ports (Store, EventBus, LlmPort)
- [ ] **P1.6** Port `dept-finance` (simplest department) to compile as .wasm
- [ ] **P1.7** Prove: same department works both compiled-in AND as WASM module
- [ ] **P1.8** Build module loader: `rashk module install <path-or-url>`
- [ ] **P1.9** CapabilityPort: WASI permission grants per module
- [ ] **P1.10** Basic SvelteKit shell (one dashboard, module list)

**Proves:** Install a .wasm module, grant it Store access, call its functions. Uninstall cleanly.

---

### Phase 2: Data Sovereignty + CRDT Sync (Weeks 5-8)
> Your data, your rules, everywhere. Works offline, syncs when connected.

- [ ] **P2.1** Integrate cr-sqlite (or chosen CRDT) into Store adapter
- [ ] **P2.2** Implement `SyncPort` with HLC-based change tracking
- [ ] **P2.3** Sync protocol: serialize changes, send over any transport
- [ ] **P2.4** Test: two instances of rashk on same machine, modify data, sync, verify merge
- [ ] **P2.5** Conflict resolution UI in SvelteKit
- [ ] **P2.6** Multi-device for single user (laptop + desktop) via local network

**Proves:** Edit data on laptop (offline), edit on desktop, connect, data merges correctly.

---

### Phase 3: Mesh Networking (Weeks 9-12)
> Machines find each other. Encrypted P2P. Zero config.

- [ ] **P3.1** Implement `MeshPort` + `MeshReceiver` traits
- [ ] **P3.2** Local network discovery via mDNS
- [ ] **P3.3** QUIC transport layer (quinn crate)
- [ ] **P3.4** Remote mesh via WireGuard (or Tailscale integration)
- [ ] **P3.5** Remote port proxying: Store adapter that delegates to another node's Store
- [ ] **P3.6** Agent delegation: "run this on my desktop" from laptop
- [ ] **P3.7** NAT traversal (STUN/TURN basics)

**Proves:** Two machines on different networks discover each other, share data, delegate agent tasks.

---

### Phase 4: Identity & Trust (Weeks 13-16)
> Web3 where it actually matters: identity, signing, reputation.

- [ ] **P4.1** Implement `IdentityPort`: local Ed25519 keypair adapter
- [ ] **P4.2** DID adapter: did:key (local) → did:sol (on-chain) progression
- [ ] **P4.3** Module signing: .wasm files signed by publisher's keypair
- [ ] **P4.4** `TrustPort`: verify module signatures, basic reputation score
- [ ] **P4.5** Verifiable Credentials: issue and verify locally
- [ ] **P4.6** Solana DID program integration (read identity from chain)

**Proves:** Install a signed module, verify publisher identity, check reputation before trusting.

---

### Phase 5: Module Registry + Token Economy (Weeks 17-24)
> The flywheel. Community-driven, self-sustaining.

- [ ] **P5.1** Cloud registry service: publish, discover, fetch .wasm modules
- [ ] **P5.2** `ModuleSource` adapter pointing to registry
- [ ] **P5.3** `PaymentPort` with Solana adapter
- [ ] **P5.4** Micropayments for premium modules (SPL token)
- [ ] **P5.5** Agent-to-agent transactions across mesh
- [ ] **P5.6** Community governance: token-weighted voting on module standards
- [ ] **P5.7** Federation: enterprises run private registries that sync with public

**Proves:** Publish a module, someone else discovers and installs it, pays in tokens.

---

### Phase 6: RUSVEL Migration (Weeks 25-30)
> Port the working RUSVEL departments into RASHK modules.

- [ ] **P6.1** Port rusvel-core domain types to rashk-core (or shared types crate)
- [ ] **P6.2** Port each wired engine (forge, code, harvest, content, flow) to .wasm modules
- [ ] **P6.3** Port SvelteKit frontend to rashk shell
- [ ] **P6.4** Port CLI/REPL/TUI surfaces
- [ ] **P6.5** Validate: everything RUSVEL does, RASHK does via modules

**Proves:** `rashk` binary replaces `rusvel` binary with all capabilities via installed modules.

---

### Phase 7: Open-Source Wrapping (Ongoing)
> Take proven open-source, wrap as RASHK modules.

Priority order (based on solo builder needs):
1. Project management (Plane/Vikunja equivalent)
2. Workflow automation (n8n equivalent)
3. CRM (EspoCRM equivalent)
4. Invoicing (Invoice Ninja equivalent)
5. Scheduling (Cal.com equivalent)
6. Team chat (Mattermost/Zulip equivalent)
7. CMS (Ghost/Strapi equivalent)
8. Support (Chatwoot equivalent)
9. Forms (Formbricks equivalent)
10. Accounting (Akaunting equivalent)

---

## 6. Growth Path

```
SINGLE USER          TEAM              COMMUNITY           ENTERPRISE
────────────         ────────          ──────────           ──────────
1 binary             N binaries        Module registry      Managed fleet
Local SQLite         CRDT mesh sync    Published .wasm      Federated mesh
Local LLM            Shared agents     Agent marketplace    Private + shared
No identity          Invite links      DID + reputation     SSO + DID hybrid
No payments          Split costs       Solana micropay      Contracts + tokens
Your machine         Your machines     Any machine          Any infrastructure
```

Same binary at every stage. The composition root wires different adapters.

---

## 7. What Makes This Unkillable

- **Runs without internet** — local-first, local LLM, local data
- **Runs without you** — open source, anyone can fork the registry
- **Runs without cloud** — P2P mesh, no central server required
- **Runs without tokens** — web3 is optional, not required
- **Gets better with network** — but doesn't require it

---

## 8. Key Decisions Log

| # | Decision | Status | ADR |
|---|----------|--------|-----|
| D1 | WASM runtime: wasmtime vs extism | PENDING | — |
| D2 | CRDT approach: cr-sqlite vs Automerge | PENDING | — |
| D3 | Mesh transport: QUIC vs WireGuard vs libp2p | PENDING | — |
| D4 | DID method: did:key vs did:web vs did:sol | PENDING | — |
| D5 | Module interface: WIT Component Model | PENDING | — |
| D6 | Token: SPL on Solana vs custom | PENDING | — |
| D7 | Port count: 17 traits (see section 3) | DRAFT | — |
| D8 | Merge RUSVEL specialized ports into fewer general ports | DRAFT | — |

---

## 9. Repo Structure (Target)

```
rashk/
├── crates/
│   ├── rashk-core/          ← 17 port traits + domain types (the constitution)
│   ├── rashk-store/         ← Store adapters: SQLite, cr-sqlite CRDT
│   ├── rashk-wasm/          ← WASM host: wasmtime/extism, WIT bindings, module loader
│   ├── rashk-mesh/          ← Mesh networking: mDNS, QUIC, WireGuard, NAT traversal
│   ├── rashk-identity/      ← Identity: Ed25519 keypair, DID, module signing
│   ├── rashk-sync/          ← CRDT sync engine: HLC, change tracking, merge protocol
│   ├── rashk-registry/      ← Module registry client + server
│   ├── rashk-payments/      ← Solana adapter for PaymentPort (optional)
│   ├── rashk-app/           ← Binary entry point, composition root
│   └── rashk-cli/           ← CLI surface
├── modules/
│   ├── wit/                 ← WIT interface definitions (shared)
│   └── examples/            ← Example .wasm modules
├── frontend/                ← SvelteKit shell
├── registry/                ← Cloud registry service (future)
├── docs/
│   ├── MASTER_PLAN.md       ← This file
│   ├── adrs/                ← Architecture Decision Records
│   └── research/            ← Research notes from Perplexity/Gemini/Claude
├── Cargo.toml
└── CLAUDE.md
```

---

## 10. Relationship to RUSVEL

RASHK is **not a rewrite** of RUSVEL. It's the **next platform** that RUSVEL's departments migrate onto.

```
RUSVEL (today)              RASHK (target)
──────────────              ──────────────
monolith binary      →     host runtime + WASM modules
50 compiled crates   →     ~10 core crates + N installable modules
local only           →     local-first + mesh + cloud
no identity          →     DID + Web3
no marketplace       →     module registry + token economy
```

The migration path:
1. Build rashk runtime (Phases 1-5)
2. Port rusvel departments as rashk WASM modules (Phase 6)
3. rashk binary replaces rusvel binary
4. Community publishes new modules (Phase 7+)

---

*Last updated: 2026-03-27*
