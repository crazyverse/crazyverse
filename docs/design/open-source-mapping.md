# RASHK — Open Source Ecosystem Mapping

> How 9 open-source projects map to the Universe Model.
> What to adopt (patterns), what to wrap (modules), what to build (native).

## The Three Strategies

For each project, RASHK has three options:

| Strategy | When | Example |
|----------|------|---------|
| **Adopt pattern** | The architecture lesson is worth more than the code | Supabase RLS → CapabilityPort design |
| **Wrap as module** | The running software becomes a RASHK service | n8n as sidecar → automation module |
| **Build native** | Reimplement the pattern in Rust for the core runtime | Tailscale mesh model → rashk-mesh |

---

## 1. Solana

**Strategy: Build native (core port adapters)**

Solana is the primary chain for the Universe Model. On-chain Rust programs (Anchor) + pure Rust SDK = same language top to bottom.

### What it gives RASHK

| Solana concept | RASHK primitive | Port |
|---------------|----------------|------|
| Accounts | `web3.account` | `ChainPort` |
| SPL Tokens / Jettons | `web3.token` | `ChainPort` |
| Transactions | `web3.transaction` | `TransactionPort` |
| Programs (Anchor) | `web3.program` / contracts | `TransactionPort` |
| PDAs | Entity registration on-chain | `IdentityPort` |
| SPL Governance / Realms | Module governance voting | Rust app module |
| did:sol | `web3.did` | `IdentityPort` |
| Ed25519 keypair | `web3.key` = `Identity.key` = `Wallet.key` | `IdentityPort` |

### Key crates

| Crate | Purpose | Maturity |
|-------|---------|----------|
| `solana-sdk` | Client, keypair, transactions | High, pure Rust |
| `anchor-client` | Anchor program interaction | High, pure Rust |
| `anchor-lang` | On-chain program framework | High |
| `spl-token` | Token operations | High |
| `solana-account-decoder` | Account state parsing | High |

### What to build

- `ChainPort` adapter: read accounts, token balances, tx history, program state
- `TransactionPort` adapter: build, simulate, sign, submit via `solana-sdk`
- `ChainListener` adapter: WebSocket subscription to account changes
- On-chain programs (Anchor): escrow, service registry, reputation SBTs
- Key material shared with `IdentityPort` — Ed25519 keypair IS the wallet

### Day-one priority: HIGH
Blockchain is core, not optional. ChainPort + TransactionPort adapters are Phase 1.

---

## 2. TON Blockchain

**Strategy: Adopt pattern (Telegram distribution) + Build adapter (secondary chain)**

TON's killer feature isn't the chain — it's **500M Telegram users** via Mini Apps with mandatory TON Connect.

### What it gives RASHK

| TON concept | RASHK use | Priority |
|-------------|-----------|----------|
| Telegram Mini Apps | SvelteKit frontend as Telegram app → instant distribution | HIGH |
| TON Connect | Wallet connection in Telegram via `@tonconnect/ui` (TS, frontend only) | HIGH |
| TON DNS (`.ton` domains) | Entity discovery — `alice.ton` resolves to ADNL address | Medium |
| Jettons (TEP-74) | Alternative payment token on TON | Medium |
| TON Storage | Decentralized BlobStore for module distribution | Low (thin ecosystem) |
| SBTs (Soulbound Tokens) | On-chain reputation / credentials | Medium |
| Layer 2 payment channels | Agent-to-agent micropayments | Low (beta) |

### Key crates

| Crate | Purpose | Maturity | Note |
|-------|---------|----------|------|
| `tonlib` | High-level TON client | Medium | **C++ FFI dependency** (tonlibjson) |
| `tonlib-core` | Cell/TL-B primitives | Medium | Required with tonlib |
| `toner` | Pure-Rust cell encoding | Low | Experimental |

### Architecture implications

- **On-chain contracts**: Written in **Tact** (TypeScript-like), NOT Rust. Async message-passing (no atomic cross-contract calls like Solana CPI).
- **Off-chain Rust client**: `tonlib-rs` wraps C++ via FFI — heavy build dependency, cross-compilation pain.
- **Frontend**: TON Connect handled entirely in SvelteKit via `@tonconnect/ui`. Rust backend verifies payments via `tonlib-rs`.
- **Dual-chain design**: `ChainPort` trait is chain-agnostic. Solana adapter for primary operations, TON adapter for Telegram distribution.

### What to build

- `TonChainAdapter` for `ChainPort` — read Jetton balances, account state
- SvelteKit + TON Connect integration for Telegram Mini App deployment
- Tact contracts for escrow/marketplace (separate from Rust, compiled separately)

### Day-one priority: MEDIUM
Solana first for on-chain Rust consistency. TON for distribution when SvelteKit frontend ships.

### New decision for MASTER_PLAN

**D9: Chain target — Solana primary + TON for Telegram distribution, or chain-agnostic from day one?**

---

## 3. n8n

**Strategy: Adopt pattern (workflow-as-JSON-DAG) + optionally wrap as sidecar**

n8n does NOT compile to WASM. But its workflow format and execution patterns are directly adoptable.

### What it gives RASHK

| n8n concept | RASHK primitive | How |
|-------------|----------------|-----|
| Workflow JSON format | `auto.workflow` definition | Serde-compatible DAG struct |
| Node type registry | `auto.node` resolved by string ID | `ModuleRegistry` lookup |
| Trigger nodes | `auto.trigger` | EventBus subscription or cron |
| `INodeExecutionData[][]` | Universal data envelope with branching | `Record` with output-index routing |
| Queue mode (Bull/Redis) | `auto.queue` | EventBus + Store |
| 500+ built-in nodes | Catalog of integrations to replicate | WASM modules or Rust app modules |
| Sub-workflow node | Workflow composition | Workflow referencing workflow by ID |

### Patterns to adopt

1. **Workflow-as-data (JSON DAG)**: Define `WorkflowDefinition` in `rashk-core` as a serde struct. Nodes reference module IDs. Connections use stable IDs (not names — n8n's mistake).
2. **Branching via output index**: `Vec<Vec<Record>>` — outer = output branch, inner = items. Simple, powerful.
3. **Node type registry**: Nodes resolved by `"rashk:http-request@1.0"` string against `ModuleRegistry`.
4. **Separate type layer from execution**: `rashk-core` defines workflow types (like `n8n-workflow`). Engine lives in a separate crate.
5. **Universal data envelope**: `Record { id, payload: Value, blobs: Vec<BlobRef>, hlc, origin }`.

### What NOT to copy

- Connection-by-name (use IDs)
- Visual position in data model (keep UI layer separate)
- JavaScript expression engine (use Rust templating or WASM-sandboxed eval)
- Credential management as engine concern (credentials = Store records + CapabilityPort)

### What to build

- `rashk-core::workflow` — `WorkflowDefinition`, `NodeDef`, `Connection`, `TriggerDef` structs
- Rust workflow executor — DAG traversal, node dispatch via `ModuleRuntime`
- n8n JSON importer — read n8n workflows, map node types to RASHK modules
- Optionally: n8n sidecar adapter for using existing n8n nodes during migration

### Day-one priority: MEDIUM
Workflow types in `rashk-core` are Phase 1. Executor is Phase 2. n8n import is Phase 7.

---

## 4. Google ADK (Agent Development Kit)

**Strategy: Adopt patterns (agent architecture)**

ADK is Python/Java/Go only, no Rust. But its agent model is the best reference for `AgentPort` design.

### What it gives RASHK

| ADK concept | RASHK primitive | How |
|-------------|----------------|-----|
| `LlmAgent` | `ai.agent` (reasoning mode) | `AgentPort::run_reasoning()` |
| `SequentialAgent` / `ParallelAgent` / `LoopAgent` | `auto.workflow` (deterministic mode) | Workflow engine, not LLM |
| `Tool` + `FunctionTool` | `ai.tool` | WIT-exported functions with schema |
| `BaseToolset` (dynamic tools) | Context-scoped tool discovery | `CapabilityPort` determines available tools per call |
| `Session` + state | `ai.context` | `Store` keyed by session ID |
| `MemoryService` | `ai.memory` | `Store` + `VectorStore` (search over past sessions) |
| `Artifact` | `ai.artifact` | `BlobStore` |
| Event stream | Canonical audit log | `EventBus` — every agent action is an event |
| Agent Card (A2A) | `platform.manifest` | `ModuleManifest` — capability discovery |
| `ToolContext` | Scoped invocation context | `CallContext` with Store + BlobStore + CapabilityGrant |
| A2A protocol | Agent-to-agent delegation | `MeshPort` + module manifest discovery |

### Patterns to adopt

1. **LLM agents vs workflow agents are distinct types.** Don't conflate reasoning with orchestration. `AgentPort` has two modes: `run_reasoning(prompt, tools, context)` and `run_workflow(definition, inputs)`.
2. **Event stream as canonical audit log.** Every tool call, LLM response, state mutation → immutable event on `EventBus`. Replayable. CRDT-friendly.
3. **ToolContext on every call.** WASM modules receive scoped context, not ambient authority. `CallContext { store, blob_store, capability_grants, invocation_id, session_id }`.
4. **Three state scopes map to existing ports.** Session = `Store` by session key. Memory = `Store` + `VectorStore`. Artifacts = `BlobStore`. No new ports needed. ADK validates RASHK's port design.
5. **Module Cards for discovery.** Every module publishes a manifest (like ADK's Agent Card). Agents query the registry to discover available tools at runtime.
6. **Dynamic tool sets.** Available tools determined at invocation time based on caller's capabilities. Not static.

### What NOT to copy

- Python-centric schema generation from docstrings (use WIT or proc macros in Rust)
- Gemini/Google Cloud coupling (RASHK's LlmPort is provider-agnostic)
- A2A HTTP transport (RASHK uses MeshPort/QUIC, but Agent Card concept is separate from transport)
- `InMemorySessionService` (RASHK is always-persistent)

### Day-one priority: HIGH (patterns)
ADK patterns directly shape `AgentPort`, `CallContext`, and event stream design. No code to wrap.

---

## 5. Claude Code

**Strategy: Adopt patterns (MCP, skills, hooks, memory, agent loop)**

Claude Code is the closest reference for how AI agents should work in RASHK's runtime.

### What it gives RASHK

| Claude Code concept | RASHK primitive | How |
|--------------------|----------------|-----|
| MCP (Model Context Protocol) | `ai.tool` + `ai.plugin` interface | Every RASHK service IS an MCP-compatible endpoint |
| MCP Tools | `ai.tool` | Module-exported functions with JSON schema |
| MCP Resources | `ai.mcp_resource` | Store records exposed to agents |
| Skills (slash commands) | `ai.skill` | Prompt + tool bundles, installable as modules |
| Hooks (before/after events) | `auto.trigger` | EventBus subscriptions |
| Memory (persistent files) | `ai.memory` | Store + VectorStore |
| Agent loop (think→tool→observe) | AgentPort core loop | `AgentPort::run_reasoning()` |
| `CallContext` | Scoped invocation | WASM module receives scoped port access |

### Key insight: MCP as the universal service interface

MCP defines how an AI agent discovers and calls external tools/resources. In RASHK's Universe Model, **every entity's service should be MCP-compatible**:

- A RASHK module exposes tools → agent discovers them via MCP
- A mesh peer exposes tools → agent discovers them via MCP over QUIC
- An external service (GitHub, Stripe) → agent discovers via MCP
- Same protocol, everywhere

This means `ModuleManifest` should include MCP-compatible tool declarations. The agent runtime speaks MCP natively.

### Patterns to adopt

1. **MCP as the tool/resource protocol.** Don't invent a custom tool interface. Use MCP's JSON-RPC + tool schema format. RASHK modules declare tools in MCP format.
2. **Skills as composable prompt+tool bundles.** A skill = `{ prompt_template, required_tools[], context_schema }`. Installable, shareable, versioned.
3. **Hooks = EventBus triggers.** "Before store.put" or "after agent.run" hooks are just EventBus subscriptions with execution priority.
4. **Memory as searchable persistent context.** Not a separate system. Store + VectorStore + a convention for memory record format.

### Day-one priority: HIGH (patterns)
MCP compatibility shapes the tool interface. No code to wrap — it's a protocol.

---

## 6. Supabase

**Strategy: Adopt patterns (critical) + wrap individual components as sidecar modules**

Supabase's component architecture maps almost 1:1 to RASHK ports.

### What it gives RASHK

| Supabase component | RASHK port/primitive | Pattern to adopt |
|-------------------|---------------------|-----------------|
| **PostgREST** (schema → REST) | `Store` query builder | Schema introspection → auto-generated API |
| **GoTrue** (auth) | `IdentityPort` | Ed25519 → signed JWT-like claim set, JWKS → DID document |
| **Realtime** (CDC → WebSocket) | `EventBus` | WAL/changelog polling → event fan-out with auth filtering |
| **RLS / WALRUS** | `CapabilityPort` + `Store` | **Authorization at the data layer, not app layer** |
| **Edge Runtime** (Rust + Deno) | `ModuleRuntime` | Isolate-per-module, capability grants at load time |
| **Supabase Wrappers** (Rust) | `ModuleRuntime` WIT | **Production WIT + wasmtime WASM module system in Rust** |
| **Storage** (S3 + TUS) | `BlobStore` | Resumable uploads, content-addressed |
| **Presence** channel | `MeshPort` | Ephemeral shared state via pub/sub |

### Critical patterns

**1. Authorization at the data layer (RLS → CapabilityPort)**

This is the single most important pattern from Supabase. RLS policies execute INSIDE Postgres — no application code can bypass them.

RASHK translation:
```
Every Store call receives a CapabilityContext:
  store.get(key, ctx)    // ctx = { caller_id, grants: ["store:read:notes/*"] }
  store.query(filter, ctx) // adapter checks grants BEFORE executing query
```

The check happens at the adapter boundary. Not in the engine. Not in the module. Not in the API handler. This is how RASHK enforces that WASM modules can only access what they're granted.

**2. Supabase Wrappers = proof our WASM approach works**

`supabase/wrappers` is a Rust crate that already does WIT + wasmtime for production WASM modules (Notion, Cal.com, etc.). Study their implementation before building RASHK's `ModuleRuntime`:
- `http.wit` — host-provided capabilities
- `routines.wit` — guest-exported functions
- `wasmtime::component::bindgen!` macro usage
- Engine/Store/Instance lifetime management

**3. CDC → EventBus**

Supabase Realtime polls Postgres WAL for changes and fans out to authorized subscribers. RASHK's equivalent:
- Store adapter emits change events to EventBus on every put/delete
- EventBus filters events per subscriber based on CapabilityPort grants (WALRUS pattern)
- Cross-node: events propagate via MeshPort

**4. GoTrue → IdentityPort claim sets**

GoTrue's JWT = `{ sub: user_id, role, custom_claims }`. RASHK's equivalent:
- `IdentityPort` produces signed claim sets (Ed25519 signature over JSON claims)
- Claims include: node_id (sub), capabilities, module grants
- Mesh peers verify claims using the signer's public key (DID document pattern)
- No shared secret needed — asymmetric verification (like JWKS)

### Day-one priority: CRITICAL (patterns)
RLS pattern and Wrappers WIT reference directly shape Phase 1 implementation.

---

## 7. NocoDB

**Strategy: Adopt pattern (metadata-over-data) + build as RASHK module**

NocoDB turns any database into a spreadsheet UI. The pattern is: **physical data stays untouched, UI concerns live in a metadata layer.**

### What it gives RASHK

| NocoDB concept | RASHK use | How |
|---------------|-----------|-----|
| Metadata layer over physical data | `schema://` namespace in Store | Views, virtual columns, display config stored as records |
| View = filter + sort + field visibility | Serializable query DSL for `Store.query()` | `ViewDefinition { filters: Vec<Filter>, sorts: Vec<Sort>, fields: Vec<FieldId> }` |
| Virtual columns (Formula, Lookup, Rollup) | Computed annotations | Module-defined overlays evaluated at read time |
| Grid / Kanban / Gallery / Calendar views | SvelteKit UI components | Reusable view components over any Store data |
| View sharing via URL | Capability-scoped read grants | CapabilityPort: "bearer can read records matching this view's filter" |
| Auto-generated REST from schema | Module manifest → API routes | ModuleRegistry reads manifest, host generates endpoints |

### Key pattern: Schema as metadata, not schema-in-core

RASHK's `Store` doesn't need to know about application schemas. Instead:
- Modules write their schema descriptions as `Store` records under `schema://`
- A table-view module reads `schema://` + physical records and renders the UI
- `SearchIndex` reads `schema://` to know which fields are searchable
- New modules add new schemas without changing `rashk-core`

### What to build

A `rashk-module-table-view` (WASM or Rust app module):
```
Imports: Store, CapabilityPort, EventBus
Exports:
  list_views(table_id) → View[]
  query_view(view_id, pagination) → RecordPage
  create_view(table_id, ViewSpec) → ViewId
  get_schema(table_id) → Column[]

SvelteKit UI:
  Grid component (virtual scroll, inline edit)
  View switcher (grid/gallery/kanban/calendar)
  Filter builder
  Column configurator
```

This one module gives every Store-backed dataset a full NocoDB-like experience.

### Day-one priority: LOW (module)
Pattern adoption (schema as metadata, serializable filter AST) is Phase 1. The actual table-view module is Phase 6-7.

---

## 8. WASM / WASI

**Strategy: Build native (core runtime layer)**

WASM is Tier 2 extensibility: sandboxed, portable, runtime-installable modules for third-party/marketplace code.

### Current state (March 2026)

| Component | Status |
|-----------|--------|
| WASI 0.2.0 (Preview 2) | **Stable** since Jan 2024 |
| WASI 0.3.0 (Preview 3) | Previews in wasmtime 37+, near-GA |
| Component Model | Stable enough for production WIT interfaces |
| wasmtime | v41.0.0, best Rust support, standards-compliant |
| wasmer | Uses non-standard WASIX — **skip** |
| extism | Wraps wasmtime but **pins to v27-31** — version lag risk |
| wit-bindgen / `bindgen!` | Production-ready for host/guest bindings |
| jco | WASM components → browser (JS + .wasm transpile) |

### Decision: wasmtime directly

| Option | Verdict | Reason |
|--------|---------|--------|
| wasmtime | **USE** | Best Rust API, Component Model first, `bindgen!` macro, WASI 0.3 preview |
| wasmer | Skip | WASIX is non-standard, RASHK modules wouldn't be portable |
| extism | Skip | Pins wasmtime < v31, blocks WASI 0.3 and latest Component Model |

### WIT interfaces for ports

Port traits → WIT is straightforward. Example `Store`:

```wit
package rashk:core@0.1.0;

interface store {
  record record-value {
    id: string,
    collection: string,
    hlc: u64,
    origin-node: string,
    data: list<u8>,
  }

  get: func(collection: string, id: string) -> result<option<record-value>, string>;
  put: func(value: record-value) -> result<_, string>;
  delete: func(collection: string, id: string) -> result<_, string>;
  query: func(collection: string, filter: string, offset: u32, limit: u32) -> result<list<record-value>, string>;
}
```

### Capability enforcement

WASI capabilities handle OS-level resources. RASHK capabilities handle app-level resources:

| RASHK capability | Enforcement |
|-----------------|-------------|
| `store:read` | Host only links `rashk:core/store` import for granted modules |
| `chain:sign` | Host only links `rashk:core/transaction` for granted modules |
| `mesh:send` | Host only links `rashk:core/mesh` for granted modules |
| CPU/memory limits | wasmtime fuel budget + memory cap per module |

The wasmtime `Linker` only defines imports for granted capabilities. Ungranteed modules fail at instantiation.

### Performance

| Metric | Value |
|--------|-------|
| Cold start | <1ms (vs 100ms+ Docker) |
| Compute vs native | 0.5–0.9x (JIT), ~1.0x (AOT) |
| Host function call | ~100-300ns per call |
| Data copy at boundary | Required for all non-scalar types (canonical ABI) |

**Optimization**: AOT-compile modules on install (`Engine::precompile_component`), cache `.cwasm` artifact.

### Browser target

Same `.wasm` does NOT run natively in browsers (no Component Model support yet). Use `jco` to transpile WASM components → JS + .wasm bundle. This is a build step, not a code change.

### Day-one priority: HIGH
wasmtime integration in `rashk-wasm` is Phase 1. WIT for Store is P0.8 exit gate.

---

## 9. Tailscale

**Strategy: Adopt pattern (mesh model) + use headscale for Phase 3 MVP**

Tailscale's architecture is the blueprint for RASHK's mesh networking.

### What it gives RASHK

| Tailscale concept | RASHK equivalent | Adaptation |
|-------------------|------------------|-----------|
| Coordination server | Mesh coordinator | Key exchange + peer list distribution |
| DERP relay | Fallback relay | For nodes behind symmetric NAT |
| Peer Relay (GA Feb 2026) | Self-hosted relay | RASHK nodes as relay infrastructure |
| WireGuard tunnels | Encrypted P2P | Data plane between nodes |
| Node key / Machine key | `IdentityPort` keypair | Stable device identity + ephemeral session keys |
| MagicDNS | Node discovery | Human-readable DID-derived names |
| ACLs (policy file) | `CapabilityPort` | Central policy → distributed enforcement |
| NAT traversal cascade | STUN → Peer Relay → DERP | Progressive fallback for connectivity |
| Headscale | Open-source coordinator | Self-hosted, Tailscale-client compatible |

### Phase 3 plan: three stages

**Stage A (MVP): Headscale as external coordinator**
- RASHK nodes use official Tailscale client daemon
- Headscale (Go, self-hosted) handles coordination
- `MeshPort` adapter wraps the local Tailscale socket
- Works immediately, production-tested crypto
- Limitation: requires Go dependency, one tailnet per headscale instance

**Stage B: boringtun + custom coordination**
- `boringtun` (Cloudflare, pure Rust WireGuard) for data plane
- Custom QUIC-based coordination protocol (using `quinn`)
- No external daemons, pure Rust
- Significant implementation complexity

**Stage C: Full mesh with DHT discovery**
- Distributed coordination (no central server)
- DHT-based peer discovery
- For community/public mesh scenarios

### Key crates

| Crate | Purpose | Maturity |
|-------|---------|----------|
| `boringtun` | Pure Rust WireGuard | Medium (Cloudflare) |
| `quinn` | QUIC transport | High |
| `mdns-sd` / `zeroconf` | Local mDNS discovery | Medium |
| headscale (Go) | Coordination server | High (external process) |

### Rust embedding status

`libtailscale` (Rust) wraps Go via CGo → C FFI. Heavy, complex cross-compilation. **Not suitable** for RASHK core. Use boringtun directly instead.

### ACL → CapabilityPort mapping

Tailscale compiles a central policy file into per-node filter rules, distributed to each node. Each node enforces its own incoming ACLs. This is exactly RASHK's model:
- Central capability policy (defined by node owner or mesh admin)
- Compiled into per-module/per-node grants
- Enforced at the receiving end (Store adapter, ModuleRuntime)
- Coordination server distributes policy, never sits in data path

### Day-one priority: LOW (Phase 3)
MeshPort trait design is Phase 0. Headscale integration is Phase 3.

---

## Synthesis: The Full Stack

```
┌─────────────────────────────────────────────────────────────────┐
│                     RASHK UNIVERSE MODEL                         │
│                                                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐    │
│  │ Business  │  │    AI    │  │Automation│  │    Web3      │    │
│  │ modules   │  │  agents  │  │ workflows│  │  contracts   │    │
│  │(NocoDB-   │  │(ADK      │  │(n8n      │  │(Solana       │    │
│  │ style UI) │  │ patterns)│  │ patterns)│  │ +TON dist)   │    │
│  └─────┬─────┘  └─────┬────┘  └─────┬────┘  └──────┬───────┘    │
│        │              │              │               │            │
│  ┌─────▼──────────────▼──────────────▼───────────────▼──────┐    │
│  │              Domain Primitives (80+)                      │    │
│  │   Typed, registered, discoverable, composable             │    │
│  └─────────────────────────┬────────────────────────────────┘    │
│                            │                                     │
│  ┌─────────────────────────▼────────────────────────────────┐    │
│  │                   Port Traits (17-19)                     │    │
│  │  Store │ EventBus │ AgentPort │ ChainPort │ MeshPort │...│    │
│  │                                                           │    │
│  │  Capability enforcement at data layer (Supabase RLS)      │    │
│  │  MCP-compatible tool interface (Claude Code)              │    │
│  │  Event stream as audit log (ADK)                          │    │
│  └─────────────────────────┬────────────────────────────────┘    │
│                            │                                     │
│  ┌─────────────────────────▼────────────────────────────────┐    │
│  │              Adapter Layer                                │    │
│  │  SQLite │ wasmtime │ Solana RPC │ tonlib │ boringtun     │    │
│  │  (Supabase Wrappers WIT model for WASM adapters)          │    │
│  └─────────────────────────┬────────────────────────────────┘    │
│                            │                                     │
│  ┌─────────────────────────▼────────────────────────────────┐    │
│  │              Infrastructure                               │    │
│  │  Rust binary │ headscale mesh │ CRDT sync │ SvelteKit UI │    │
│  │  (Tailscale mesh model for networking)                    │    │
│  └──────────────────────────────────────────────────────────┘    │
│                                                                  │
│  Distribution: Telegram Mini App (TON Connect) + Native + Browser│
└─────────────────────────────────────────────────────────────────┘
```

---

## Decision Impact on MASTER_PLAN

| Decision | Recommendation | Source |
|----------|---------------|--------|
| D1: WASM runtime | **wasmtime** directly, `bindgen!` macro | WASM research |
| D3: Mesh transport | **Headscale MVP → boringtun + quinn** | Tailscale research |
| D5: Module interface | **WIT Component Model**, WASI 0.2 | WASM research |
| D6: Token | **SPL on Solana** primary, Jettons on TON for Telegram | TON research |
| **D9 (new)**: Chain target | **Solana primary + TON secondary** (Telegram distribution) | TON research |
| **D10 (new)**: Tool protocol | **MCP-compatible** tool declarations in module manifests | Claude Code + ADK |
| **D11 (new)**: Authorization model | **Capability enforcement at data layer** (Supabase RLS pattern) | Supabase research |

---

## Phase 1 Priority Actions from This Research

| Action | Source | Impact |
|--------|--------|--------|
| Study `supabase/wrappers` WIT + wasmtime implementation | Supabase | Directly informs `ModuleRuntime` |
| Design `CapabilityContext` passed to every Store call | Supabase RLS | Enforces security model |
| Define `WorkflowDefinition` serde struct in `rashk-core` | n8n | Automation primitives |
| Add `CallContext` to WASM module invocations | ADK | Scoped authority |
| Design `ModuleManifest` with MCP-compatible tool declarations | Claude Code + ADK | Discovery protocol |
| Expand `ChainPort` + `TransactionPort` + `ChainListener` | Solana | Blockchain core ports |
| Prototype WIT for Store port | WASM/Supabase Wrappers | P0.8 exit gate |

---

*Created: 2026-03-29*
*Sources: Research agents analyzing TON, n8n, ADK, Supabase, NocoDB, Tailscale, WASM/WASI, Claude Code, Solana*
