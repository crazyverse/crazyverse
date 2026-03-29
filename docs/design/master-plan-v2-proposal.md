# RASHK Master Plan v2 — Proposal

> Sovereign computing stack. From OS to marketplace.
> Every entity, service, and transaction is a first-class primitive.
> Agents discover, compose, execute, and transact across a decentralized mesh.
> Contracts on-chain. Payments in code. Code rules all.

**Status:** Proposal — pending review and adoption
**Supersedes:** docs/MASTER_PLAN.md (v1, 2026-03-27)
**Author:** Mehdi Baneshi + Claude Code
**Date:** 2026-03-29

---

## 1. What RASHK Is

A **sovereign computing stack** — from OS to marketplace — where:

- Everything is an **entity** with cryptographic identity
- Entities publish **services** that are discoverable by anyone
- **Agents** (AI-powered) discover, negotiate, execute, and transact autonomously
- **Contracts** are on-chain — terms are code, enforcement is automatic
- **Payments** flow transparently via blockchain — no middlemen, no 30% cut
- The platform is **owned by its participants** — not a company

RASHK is not an app. Not a platform in the SaaS sense. It's the **infrastructure for an agent economy** where anyone in the world can offer a product or service, and anyone can discover and purchase it.

---

## 2. The Principles

### Build our own, inspired by open source

Every technology is a **pattern to learn from**, not a dependency to rely on. Telegram can change rules. Solana can fork. Any external platform can restrict access. RASHK's port traits abstract ALL of this. We build our own implementations that happen to be compatible with the ecosystem.

| We learn from | The pattern | We build |
|--------------|-------------|----------|
| Solana | PDA registration, atomic transactions, Anchor | Our own chain abstraction (ChainPort) |
| TON | Telegram distribution, async contracts, .ton DNS | Our own discovery protocol |
| Tailscale | Coordination + WireGuard + NAT cascade | Our own mesh in Rust |
| n8n | Workflow-as-JSON-DAG, node registry | Our own workflow engine in Rust |
| Chromium | Protocol handlers, built-in wallet | Our own Tauri shell |
| NixOS | Immutable OS, declarative config | Our own OS image |
| Supabase | RLS, CDC→events, WIT WASM host | Our own capability system + module runtime |
| NocoDB | Metadata-over-data, views as lenses | Our own data UI layer |
| Google ADK | Event stream, ToolContext, Agent Card | Our own agent runtime |
| Claude Code | MCP, skills, hooks, memory, agent loop | Our own AI building service |

### AI is the team, not a tool

Claude Code writes code. RUSVEL creates flows and playbooks 24/7. The constraint isn't manpower — it's **architectural clarity**. Research thoroughly, design clearly, then AI builds at scale. This is why Phase 0 matters.

### Research prevents refactoring

Narrowing scope leads to infinite refactoring. Thorough research + clear architecture = build once, expand forever. The 11 Perplexity research prompts answered every major technical question before a line of implementation code.

### Two-tiered extensibility

- **Tier 1 — Rust app modules:** Compiled-in, full port access, native speed. Like Django apps. For first-party and trusted code.
- **Tier 2 — WASM modules:** Sandboxed, capability-granted, runtime-installable. For third-party marketplace code and browser portability.

### Web3 is core, not optional

Finance and blockchain are primary data sources — as fundamental as storage or events. The keypair that signs modules IS the wallet that holds tokens IS the identity. One key, everywhere.

---

## 3. The Full Stack

```
Layer 7: MARKETPLACE ── anyone publishes, discovers, contracts on-chain
Layer 6: APPLICATIONS ── Rust app modules + WASM modules
Layer 5: SERVICES ──── Communication, Learning, Building, Commerce
Layer 4: RUNTIME ───── rashk binary (ports, agents, events, sync) ← CORE
Layer 3: WEB3+MESH ── chain abstraction + P2P encrypted mesh
Layer 2: SHELL ─────── Tauri app (Rust backend + system WebView)
Layer 1: OS ────────── NixOS-based immutable appliance image
```

Every layer is independently useful. Layer 4 is the core product — everything below supports it, everything above runs on it. Layers 1-2 come last as packaging.

---

## 4. The Universe Model

### Core concepts

**Entity** — anything with identity: person, business, AI agent, machine, module. Has: cryptographic identity (DID + keypair), discoverability, capabilities, reputation, agency.

**Service** — what an entity offers. Has: definition (inputs/outputs/SLA), price, version. Is composable — one service can use other services.

**Contract** — how entities agree. On-chain code. Has: terms, escrow, deadline, milestones. Lifecycle: propose → negotiate → agree → escrow → execute → deliver → verify → pay → rate.

**Agent** — an economic actor (AI-powered). Can: discover services, evaluate options, negotiate terms, execute work, sign transactions, report results, learn from outcomes.

**Payment** — on-chain value transfer. Programmable: escrow, milestone-based, streaming, subscription. Transparent: every payment is auditable. Instant: chain finality in seconds.

### The value layers

```
Economy    ── tokens + reputation make it self-sustaining
Agent      ── AI makes it autonomous (24/7 operation)
Contract   ── chain makes it trustless (no middlemen)
Service    ── composability makes it infinite
Entity     ── identity makes it sovereign
Compute    ── mesh makes it unstoppable
```

---

## 5. Domain Primitives (6 domains, 80+ types)

Every primitive is: registered, discoverable, CRUD-able, composable, synced via CRDT, event-emitting.

### AI Domain
Provider, Model, Completion, Agent, Tool, Memory, Artifact, Skill, Plugin, Prompt, Context, Embedding, Chain-of-thought, McpServer, McpResource

### Automation Domain
Workflow, Flow, Node, Trigger, Action, Condition, Schedule, Webhook, Queue, Job, Retry, Template

### Web3 Domain
Chain, Wallet, Key, Account, Token, NFT, Transaction, Program/Contract, Signature, Block, ChainEvent, DID, Credential, RPC

### Business Domain
Project, Task, Document, Invoice, Contact, Lead, Pipeline, Campaign, Content, Form, Calendar, Ticket, Account (financial), Report

### Platform Domain
Module, Manifest, Registry, Marketplace, Rating, License, Capability, Sandbox, Version, Hook

### Network Domain
Node, Peer, Mesh, Tunnel, Sync, Conflict, Replica, Discovery, Delegation, Subscription

### Universal Primitive Protocol

Every primitive follows the same contract:

```rust
trait Primitive: Serialize + Deserialize + Send + Sync {
    fn id(&self) -> &PrimitiveId;
    fn kind(&self) -> &str;           // "ai.agent", "web3.token", "biz.invoice"
    fn hlc(&self) -> HlcTimestamp;    // For CRDT ordering
    fn origin_node(&self) -> &NodeId; // For CRDT merge
    fn metadata(&self) -> &Value;
}
```

Primitives are MANY. Ports are FEW. That's the leverage.

---

## 6. Port Traits (revised)

### Validated patterns applied

| Pattern | Source | RASHK implementation |
|---------|--------|---------------------|
| Capability at data layer | Supabase RLS/WALRUS | Every Store call receives CapabilityContext |
| WIT + wasmtime for modules | Supabase Wrappers | ModuleRuntime uses bindgen! macro |
| Event stream as audit log | Google ADK | EventBus records every agent action |
| Workflow-as-data | n8n | WorkflowDefinition is a serde struct |
| MCP-compatible tools | Claude Code | Module manifests declare tools in MCP format |
| Metadata-over-data | NocoDB | schema:// namespace in Store |
| Coordination + tunnel cascade | Tailscale | MeshPort with progressive fallback |
| On-chain transparent splits | Crypto marketplaces | PaymentSplitter contracts |

### Core ports (revised — 19 traits)

| # | Port | Layer | Purpose |
|---|------|-------|---------|
| **Storage** | | | |
| 1 | `Store` | Runtime | Key-value + document records + CapabilityContext |
| 2 | `BlobStore` | Runtime | Files, media, WASM binaries |
| 3 | `SearchIndex` | Runtime | Full-text search |
| 4 | `VectorStore` | Runtime | Embedding similarity search |
| **Events** | | | |
| 5 | `EventBus` | Runtime | Pub/sub + replay + audit log |
| **AI** | | | |
| 6 | `LlmPort` | Runtime | Completion, embeddings, streaming, tool-use mode |
| 7 | `AgentPort` | Runtime | Reasoning mode + workflow mode + event stream output |
| **Blockchain** | | | |
| 8 | `ChainPort` | Web3 | Read on-chain state: accounts, tokens, tx history |
| 9 | `TransactionPort` | Web3 | Build, simulate, sign, submit transactions |
| 10 | `ChainListener` | Web3 | Subscribe to on-chain events → feeds EventBus |
| **Modules** | | | |
| 11 | `ModuleRegistry` | Platform | Install, uninstall, list, get manifest |
| 12 | `ModuleRuntime` | Platform | Load, call, unload WASM modules (wasmtime) |
| 13 | `CapabilityPort` | Platform | Grant, revoke, check permissions |
| **Mesh** | | | |
| 14 | `MeshPort` | Network | Discover, connect, send, receive, subscribe |
| **Sync** | | | |
| 15 | `SyncPort` | Network | CRDT merge, HLC tracking, change propagation |
| **Identity** | | | |
| 16 | `IdentityPort` | Identity | Keypair, DID, signing, verification, seed management |
| 17 | `TrustPort` | Identity | Module verification, reputation scores, credential check |

### Removed from core (become modules)

| Former port | Now | Reason |
|-------------|-----|--------|
| `PaymentPort` | Rust app module composing ChainPort + TransactionPort | Payment is a use case, not infrastructure |
| `ModuleSource` | Deferred to Phase 5 | Not needed until marketplace exists |
| `MeshReceiver` | Merged into MeshPort | One trait with send + receive + subscribe |

---

## 7. Architecture Decisions (resolved)

All 11 decisions informed by Perplexity research (docs/research/P1-P11).

| # | Decision | Resolution | Confidence | Source |
|---|----------|-----------|------------|--------|
| D1 | WASM runtime | **wasmtime** directly, `bindgen!` macro. Not extism (version lag), not wasmer (non-standard WASIX). | HIGH | P1 |
| D2 | CRDT approach | **Custom Rust CRDT layer** (HLC + LWW registers over plain SQLite). cr-sqlite is pre-1.0/risky. ElectricSQL is Postgres-centric. Own implementation = full control. | MEDIUM | P3 |
| D3 | Mesh transport | **Own implementation** inspired by Tailscale: coordination protocol + GotaTun/defguard_wireguard_rs (pure Rust WireGuard) + QUIC (quinn) for control. Headscale for compatibility testing only. | HIGH | P4 |
| D4 | DID method | **did:key** (day 1, local) + **did:pkh** (wallet-as-DID) → did:web (DNS-backed) → did:sol (Solana-native). SpruceID Rust stack (ssi + didkit). | HIGH | P11 |
| D5 | Module interface | **WIT Component Model**, WASI 0.2. Study Supabase Wrappers as reference. Batch operations, versioned WIT packages. | HIGH | P1 |
| D6 | Token/payment | **Own token design** on Solana (SPL). Stablecoin support. On-chain payment splitter contracts. Inspired by Shopify's tiered model but transparent. | HIGH | P2, P10 |
| D7 | Port count | **19 core ports** (revised from 17). Added ChainPort, TransactionPort, ChainListener. Removed PaymentPort (→ module), ModuleSource (→ deferred), merged MeshReceiver into MeshPort. | HIGH | All |
| D8 | Port consolidation | Confirmed — fewer general ports, specialized concerns become modules. | HIGH | All |
| D9 | Chain strategy | **Chain-agnostic** via ChainPort adapter. Solana adapter first (Rust-native, PDA model). TON adapter for Telegram distribution. More chains via community adapters. No dependency on any single chain. | HIGH | P2, P5 |
| D10 | Tool protocol | **MCP-compatible** tool declarations in module manifests. Agent Cards (A2A-inspired) for module discovery. | HIGH | P6 |
| D11 | Auth model | **Capability at data layer** (Supabase RLS pattern). CapabilityContext on every Store call. Wasmtime Linker enforces grants at module instantiation. | HIGH | P1, Supabase research |

### New decisions from full-stack research

| # | Decision | Resolution | Source |
|---|----------|-----------|--------|
| D12 | Shell/browser | **Tauri** (Rust backend + system WebView). NOT Chromium fork (needs 50+ engineers). Custom protocols, agent sidebar, wallet — all from Rust. | P8 |
| D13 | OS image | **NixOS-based**. Declarative, immutable, reproducible. nixos-generators for ISO/VM/Pi/container images. Colmena for fleet management. | P9 |
| D14 | Marketplace economics | **0-2% first $X, 5-10% above**, encoded in smart contracts. 90/5/5 splits (author/treasury/referrer). Stablecoin + native token. DAO governance for fee changes. | P10 |
| D15 | Wallet architecture | **One mnemonic, multi-chain derivation**. Solana: m/44'/501'/{i}'/0' (Ed25519). TON: m/44'/607'/... (Ed25519, different path). EVM: m/44'/60'/... (secp256k1). IdentityPort manages seed. | P7 |

---

## 8. Research Questions (all resolved)

| # | Question | Status | Finding | Doc |
|---|----------|--------|---------|-----|
| P1 | WASM Component Model in production | DONE | wasmtime + bindgen! is the path. Supabase Wrappers is production reference. | P1-wasm-component-model.md |
| P2 | Solana escrow & marketplace | DONE | PDA-based entity/service/contract/milestone registration. 15K lamports for 3-step escrow. | P2-solana-escrow-marketplace.md |
| P3 | cr-sqlite readiness | DONE | Pre-1.0, risky. Custom CRDT layer recommended. | P3-cr-sqlite-readiness.md |
| P4 | Headscale + WireGuard mesh | DONE | 40-100 nodes on single VPS. GotaTun (Mullvad) for Rust WireGuard. defguard_wireguard_rs for embedding. | P4-headscale-wireguard-mesh.md |
| P5 | Telegram Mini Apps | DONE | SvelteKit works. TON Connect mandatory. Stars ~30% on digital goods. Same codebase runs as PWA outside Telegram. | P5-telegram-mini-apps.md |
| P6 | Agent economy | DONE | Emerging. A2A is enterprise workflow, not open market. Fetch.ai piloted AI-to-AI payments. RASHK would be early mover. | P6-agent-economy.md |
| P7 | Multi-chain wallet | DONE | One seed, per-chain derivation. Solana + TON both Ed25519 but different paths. Never reuse raw keys across chains. | P7-multi-chain-wallet.md |
| P8 | Chromium fork | DONE | NOT feasible for small team. Tauri is the right approach. | P8-chromium-fork.md |
| P9 | NixOS appliance | DONE | Generators upstreamed. Fleet via Colmena. Pi supported. Most flexible immutable OS. | P9-nixos-appliance.md |
| P10 | Marketplace economics | DONE | 5-10% sweet spot. On-chain splits. Transparent fees. Multiple revenue models. | P10-marketplace-economics.md |
| P11 | DID methods | DONE | did:key + did:pkh for day 1. SpruceID Rust stack. did:ion is dead. | P11-did-methods.md |

---

## 9. Implementation Phases (revised)

### Phase 0: Design & Research — NEARLY COMPLETE

- [x] Vision document (docs/design/vision.md)
- [x] Universe model (docs/design/universe.md)
- [x] Domain primitives (docs/design/domain-model.md)
- [x] Open-source mapping (docs/design/open-source-mapping.md)
- [x] Full stack architecture (docs/design/full-stack.md)
- [x] Research all 11 questions (docs/research/P1-P11)
- [x] Resolve all 15 architecture decisions
- [ ] Write ADRs for each decision
- [ ] Revise port traits based on decisions
- [ ] Prototype WIT interface for Store port
- [ ] Validate: Rust app module + WASM module both call Store through same trait

**Exit criteria:** ADRs written. WIT prototype validates the approach. Port traits finalized.

### Phase 1: Runtime + Full Architecture (simplest implementation of each layer)

> Not one narrow vertical. The full architecture with the simplest possible implementation of every layer.

- [ ] **rashk-core**: 19 port traits + Primitive protocol + domain types
- [ ] **rashk-store**: SQLite adapter with CapabilityContext (Supabase RLS pattern)
- [ ] **rashk-events**: In-memory EventBus with audit log (ADK pattern)
- [ ] **rashk-agent**: AgentPort with one LLM provider + MCP tool interface
- [ ] **rashk-chain**: ChainPort + TransactionPort adapters for Solana devnet
- [ ] **rashk-identity**: IdentityPort with did:key + did:pkh + seed management
- [ ] **rashk-wasm**: wasmtime host + WIT for Store (Supabase Wrappers pattern)
- [ ] **rashk-workflow**: WorkflowDefinition + DAG executor (n8n pattern)
- [ ] **rashk-app**: Composition root, boot sequence (RUSVEL pattern)
- [ ] **rashk-cli**: CLI surface

**Proves:** Entity registers → Service listed → Agent discovers → Contract on-chain → Agent executes → Payment settles → Reputation updated. The full loop.

### Phase 2: Sync + Mesh

- [ ] Custom CRDT layer (HLC + LWW over SQLite)
- [ ] SyncPort implementation
- [ ] MeshPort with WireGuard tunnels (GotaTun/defguard)
- [ ] Coordination protocol (inspired by Tailscale, our implementation)
- [ ] mDNS local discovery
- [ ] Two nodes sync and mesh

**Proves:** Edit on laptop (offline), edit on desktop, connect, data merges. Agents delegate across nodes.

### Phase 3: Shell + Frontend

- [ ] Tauri app with Rust backend + system WebView
- [ ] SvelteKit dashboard (modules, mesh status, wallet)
- [ ] Agent sidebar
- [ ] Built-in wallet (Solana + TON, one seed)
- [ ] rashk:// protocol handler
- [ ] Telegram Mini App deployment (same SvelteKit codebase)

**Proves:** Full UI experience. Works as desktop app (Tauri), web app (browser), and Telegram Mini App.

### Phase 4: Identity + Trust + Chain

- [ ] Full DID progression: did:key → did:pkh → did:web → did:sol
- [ ] Verifiable Credentials (SpruceID ssi + didkit)
- [ ] Module signing with Ed25519
- [ ] TrustPort: verify signatures, reputation scores from on-chain history
- [ ] ChainListener: on-chain events → EventBus
- [ ] Solana mainnet deployment (escrow, registry, reputation programs)
- [ ] TON adapter for ChainPort (Telegram distribution path)

**Proves:** Install a signed module, verify publisher identity, check on-chain reputation. On-chain service registry.

### Phase 5: Marketplace + Economy

- [ ] Module registry (local + cloud)
- [ ] On-chain payment splitter contracts (90/5/5)
- [ ] Marketplace discovery (search, filter, ratings)
- [ ] Multiple revenue models (one-time, subscription, usage-based)
- [ ] Stablecoin support
- [ ] DAO governance for fee schedule
- [ ] Federation: private registries that sync with public

**Proves:** Publish a module. Someone discovers, installs, pays. Builder earns. Transparent.

### Phase 6: RUSVEL Migration

- [ ] Port RUSVEL domain types to rashk-core
- [ ] Port each wired engine (forge, code, harvest, content, flow) as Rust app modules
- [ ] Port SvelteKit frontend
- [ ] Port CLI/REPL/TUI surfaces
- [ ] rashk binary replaces rusvel binary

### Phase 7: Services

- [ ] **Communication Service** — messaging primitives, channels, notifications (inspired by Telegram)
- [ ] **Learning Service** — courses, exercises, certifications as VCs, creator monetization
- [ ] **Building Service** — AI-assisted module creation (Claude Code patterns), scaffold → test → publish
- [ ] **Commerce Service** — entity profiles, service listings, agent-assisted discovery, on-chain contracts

### Phase 8: Shell Evolution

- [ ] Tauri app → deeper OS integration
- [ ] Custom protocol handlers (entity://, service://, module://)
- [ ] Agent DevTools panel
- [ ] Mesh inspector
- [ ] Chain explorer

### Phase 9: OS Image

- [ ] NixOS-based RashkOS
- [ ] Desktop, server, Pi, VM, container, WSL images
- [ ] Declarative config (one .nix file = entire node)
- [ ] Fleet management via Colmena + Git-ops
- [ ] Auto-update with atomic rollback

### Phase 10: Open-Source Module Ecosystem

Community builds modules for every business need:
1. Project management (Plane/Vikunja patterns)
2. Workflow automation (n8n patterns)
3. CRM (EspoCRM patterns)
4. Invoicing (Invoice Ninja patterns)
5. Scheduling (Cal.com patterns)
6. Team chat (Mattermost patterns)
7. CMS (Ghost/Strapi patterns)
8. Support (Chatwoot patterns)
9. Forms (Formbricks patterns)
10. Accounting (Akaunting patterns)
11. DeFi management
12. DAO governance
13. And anything else the community builds

---

## 10. Repo Structure (revised)

```
rashk/
├── crates/
│   ├── rashk-core/          ← 19 port traits + Primitive protocol + domain types
│   ├── rashk-store/         ← SQLite adapter with CapabilityContext
│   ├── rashk-events/        ← EventBus with audit log
│   ├── rashk-agent/         ← AgentPort + MCP tool interface
│   ├── rashk-chain/         ← ChainPort + TransactionPort + ChainListener
│   ├── rashk-identity/      ← IdentityPort + DID + seed management + VC
│   ├── rashk-wasm/          ← wasmtime host + WIT bindings
│   ├── rashk-workflow/      ← DAG executor + WorkflowDefinition
│   ├── rashk-mesh/          ← MeshPort + WireGuard + coordination
│   ├── rashk-sync/          ← Custom CRDT + HLC + SyncPort
│   ├── rashk-app/           ← Binary entry point, composition root
│   └── rashk-cli/           ← CLI surface
├── modules/
│   ├── wit/                 ← WIT interface definitions
│   ├── payment/             ← Payment module (Rust app, composes ChainPort)
│   └── examples/            ← Example modules
├── programs/                ← Solana on-chain programs (Anchor)
│   ├── escrow/
│   ├── registry/
│   └── reputation/
├── frontend/                ← SvelteKit (browser + Telegram Mini App + Tauri WebView)
├── shell/                   ← Tauri desktop app
├── os/                      ← NixOS configuration + image generators
├── docs/
│   ├── MASTER_PLAN.md       ← This file (when adopted)
│   ├── design/              ← Vision, universe model, domain model, architecture
│   ├── research/            ← P1-P11 Perplexity findings
│   ├── adrs/                ← Architecture Decision Records
│   └── reference/           ← Port reference pages, WIT docs
├── Cargo.toml
└── CLAUDE.md
```

---

## 11. The Participation Path

### Anyone in the world can:

**Use** — Install rashk. Create identity. Install modules. Work with agents.

**Build** — Open Building Service. Describe a module. AI scaffolds, tests, publishes. Earn when others install.

**Offer services** — Register as entity. List skills. Agent handles discovery + scheduling. Get paid on-chain.

**Learn** — Take courses on Learning Service. Earn Verifiable Credentials. Build portfolio.

**Govern** — Hold tokens. Vote on standards. Propose changes. Fund grants.

---

## 12. What Makes This Unkillable

- **Runs without internet** — local-first, local LLM, local data
- **Runs without any single chain** — ChainPort adapts to any blockchain
- **Runs without any platform** — no dependency on Telegram, Solana, or any service
- **Runs without cloud** — P2P mesh, no central server required
- **Runs without tokens** — economy is optional, runtime is not
- **Gets better with network** — but doesn't require it
- **Cannot be killed by legal action against one chain/platform** — abstractions protect everything

---

## 13. Design Documents

| Document | Location | Purpose |
|----------|----------|---------|
| Vision | docs/design/vision.md | The problem and insight |
| Universe Model | docs/design/universe.md | Entities, services, contracts, agents |
| Domain Model | docs/design/domain-model.md | 80+ primitives across 6 domains |
| Open-Source Mapping | docs/design/open-source-mapping.md | 9 projects → patterns to adopt |
| Full Stack | docs/design/full-stack.md | OS → marketplace, all 7 layers |
| This Proposal | docs/design/master-plan-v2-proposal.md | Consolidated plan |
| Research (11 docs) | docs/research/P1-P11 | Perplexity findings |

---

*Proposed: 2026-03-29*
*Status: Draft proposal — to be reviewed, refined, and adopted as the new MASTER_PLAN.md*
