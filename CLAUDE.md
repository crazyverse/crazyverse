# RASHK-RUS — Project Instructions

> RASHK runtime powered by RUSVEL's proven crate ecosystem.
> Two cores, one binary: rashk-core (distributed/decentralized ports) + rusvel-core (application ports).
> RUSVEL engines become RASHK's first WASM modules.

## Origin

- **RUSVEL** (`/Users/bm/rusvel`) — 54 crates, working virtual agency app, proven hexagonal architecture
- **RASHK** (`/Users/bm/rashk`) — Design docs + core scaffold for decentralized capability runtime
- **RASHK-RUS** (this repo) — Unified build: RASHK's vision + RUSVEL's working code, developed independently

## Current Phase: 1 — Runtime + WASM Module System

RASHK design docs are in `docs/`. RUSVEL crates are working code from the original repo.
Goal: make RUSVEL engines callable as WASM modules through rashk-core's `ModuleRuntime` port.

## Quick Commands

```bash
cargo build                    # Build all 60 workspace members
cargo test                     # Run full test suite
cargo run                      # Start RUSVEL app on :3000 (works today)
cargo run -- --mcp             # MCP server (stdio JSON-RPC)
cargo run -- --tui             # TUI dashboard
cargo run -- shell             # Interactive REPL
```

## Architecture

Two hexagonal cores coexisting:

### rashk-core (17 ports — distributed layer)
Store, BlobStore, SearchIndex, VectorStore, SyncPort, EventBus,
ModuleRegistry, ModuleSource, ModuleRuntime, MeshPort, MeshReceiver,
IdentityPort, TrustPort, CapabilityPort, LlmPort, AgentPort, PaymentPort

### rusvel-core (21 ports — application layer)
LlmPort, AgentPort, EventPort, StoragePort (5 sub-stores), MemoryPort,
SessionPort, ToolPort, JobPort, AuthPort, ConfigPort, EmbeddingPort,
VectorStorePort, ChannelPort, BrowserPort, DeployPort, TerminalPort,
CronPort, WebhookPort, MetricStore

### The Bridge (Phase 2+)
RUSVEL engines depend on rusvel-core ports. RASHK adapters implement rashk-core ports.
The bridge: rashk adapters that delegate to rusvel adapters (or vice versa), allowing
engines to run through either core depending on context (local vs distributed).

## Workspace Layout (60 crates)

```
crates/
├── rashk-core/           17 universal port traits (distributed)
├── rashk-store/          Storage adapter (SQLite + CRDT-ready)
├── rashk-wasm/           WASM host (wasmtime, WIT bindings)
├── rashk-mesh/           Mesh networking (mDNS, QUIC)
├── rashk-identity/       Identity (Ed25519 keypair, DID)
├── rashk-app/            RASHK binary entry point
├── rusvel-core/          21 application port traits
├── rusvel-*/             ~20 adapter crates (db, llm, agent, etc.)
├── *-engine/             13 domain engines
├── dept-*/               14 department apps (DepartmentApp pattern)
├── rusvel-api/           Axum HTTP API
├── rusvel-cli/           3-tier CLI (one-shot + REPL + TUI)
├── rusvel-mcp/           MCP server
├── rusvel-tui/           TUI dashboard
└── rusvel-app/           RUSVEL binary entry point
frontend/                 SvelteKit 5 + Tailwind 4
docs/                     RASHK design docs + research
```

## Key Rules

1. **Engines never import adapter crates.** They depend only on port traits.
2. **Engines never call LlmPort directly.** They use AgentPort (ADR-009).
3. **rashk-core ports must work across ALL targets:** local, WASM, mesh, cloud, on-chain, browser.
4. **All rashk-core types must be serde Serializable** for WASM/mesh boundary crossing.
5. **No filesystem paths in rashk-core port signatures.** Use string keys or URIs.
6. **No runtime-specific types in rashk-core ports.** No `tokio::sync::broadcast`, no `std::path::Path`.
7. **Each crate < 2000 lines.** Single responsibility.
8. **Every rashk-core record has HLC + origin_node** for CRDT merge readiness.
9. **WASM modules get explicit capability grants.** No ambient authority.
10. **NEVER use npm.** Use `pnpm` for all frontend/Node.js work.
11. **NEVER use pip/pip3.** Use `uv` for all Python work.

## Phases

1. **Phase 1** — Runtime + WASM Module System (NOW)
2. **Phase 2** — Data Sovereignty + CRDT Sync
3. **Phase 3** — Mesh Networking
4. **Phase 4** — Identity & Trust
5. **Phase 5** — Module Registry + Token Economy
6. **Phase 6** — RUSVEL Engine Migration to WASM modules
7. **Phase 7** — Open-Source Wrapping

Full details: `docs/MASTER_PLAN.md`

## Package Managers

- **cargo** for Rust
- **pnpm** (never npm) for frontend
- **uv** (never pip) for Python

## Code Style

- Rust: idiomatic, thiserror for lib errors, anyhow in binaries
- Concise over verbose, no unnecessary comments
- Small functions, single responsibility
- Conventional commits: feat:, fix:, refactor:, docs:, chore:, test:
- Atomic commits — one logical change per commit

## Testing

```bash
cargo test                     # Full workspace
cargo test -p rashk-core       # RASHK core
cargo test -p rusvel-core      # RUSVEL core
cargo test -p forge-engine     # Single engine
```

## Stack

- Rust edition 2024, SQLite WAL, Axum, Clap 4, reedline, ratatui, tokio
- SvelteKit 5, Tailwind CSS 4, pnpm
- Python scripts: uv
- LLM: Ollama, Claude API, Claude CLI, OpenAI
- Vector DB: LanceDB + Arrow
- WASM: wasmtime + WIT Component Model
- Identity: Ed25519 + DID
- Mesh: QUIC + mDNS
