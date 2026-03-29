# RASHK Domain Model — First-Class Primitives

> Every primitive across every domain is a registered, discoverable, composable entity.
> The port/adapter layer is the foundation. The primitive layer is what everything talks to.

## The Core Idea

Rashk doesn't "integrate" AI, automation, Web3, and business tools. It defines every concept from every domain as a **first-class primitive** with a universal contract:

- **Registry** — register, deregister, list, get
- **Discovery** — search, filter, browse, recommend
- **CRUD** — create, read, update, delete
- **Lifecycle** — init, start, pause, resume, stop, archive
- **Events** — every state change emits to the EventBus
- **Composability** — any primitive can reference, contain, or trigger any other

This means a `Workflow` node can invoke an `Agent`, which reads `ChainState`, updates a `Project`, creates an `Invoice`, and signs a `Transaction` — all through the same primitive protocol.

---

## Domain Primitives

### 1. AI Domain

Everything an AI system needs to think, act, remember, and extend.

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Provider` | LLM service endpoint | OpenAI, Anthropic, Ollama, local GGUF |
| `Model` | Specific model on a provider | claude-opus-4-20250115, llama-3, gpt-4o |
| `Completion` | A single LLM call (input/output/tokens/cost) | Logged, replayable, cacheable |
| `Agent` | Autonomous tool-using AI entity | Finance agent, code agent, research agent |
| `Tool` | Callable function an agent can invoke | store.query, chain.balance, mesh.send |
| `Memory` | Persistent context for agents | Conversation history, learned facts, embeddings |
| `Artifact` | Structured output from agent work | Generated report, code file, analysis |
| `Skill` | Reusable prompt+tool bundle | "Analyze codebase", "Generate invoice" |
| `Plugin` | External integration for agents | MCP server, API connector, browser tool |
| `Prompt` | Templated instruction with variables | System prompts, task templates |
| `Context` | Assembled input for a completion | Documents + memory + tools → prompt |
| `Embedding` | Vector representation of content | For semantic search, RAG, clustering |
| `Chain` (of thought) | Multi-step reasoning trace | Logged for debugging and learning |
| `McpServer` | Model Context Protocol endpoint | Stdio or HTTP, tool/resource provider |
| `McpResource` | Data exposed via MCP | Files, DB rows, chain state |

**Cross-domain links:**
- `Agent` uses `Tool` → `Tool` calls any port (Store, Chain, Mesh...)
- `Agent` has `Memory` → `Memory` is stored via `Store` + `VectorStore`
- `Agent` produces `Artifact` → `Artifact` can be a `Document` (Business) or `Transaction` (Web3)
- `Skill` bundles `Prompt` + `Tool[]` → installable as a `Module` (Platform)

---

### 2. Automation Domain

Everything needed to wire actions into flows that run without human intervention.

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Workflow` | Named automation with trigger + steps | "On invoice paid, update project" |
| `Flow` | DAG of connected nodes (the graph) | Visual or code-defined |
| `Node` | Single step in a flow | Action, condition, transform, wait |
| `Trigger` | What starts a flow | Event, cron, webhook, chain event, manual |
| `Action` | What a node does | Call agent, write store, send transaction |
| `Condition` | Branch logic | If balance > X, if status == "paid" |
| `Schedule` | Cron/interval timing | "Every 6 hours", "Mon 9am" |
| `Webhook` | HTTP endpoint that triggers a flow | Inbound from Stripe, GitHub, etc. |
| `Queue` | Ordered list of pending work | Job queue with retry and priority |
| `Job` | A single queued unit of work | Background task with status tracking |
| `Retry` | Failure recovery policy | Exponential backoff, max attempts |
| `Template` | Reusable flow definition | "Onboarding flow", "Invoice cycle" |

**Cross-domain links:**
- `Trigger` listens to `Event` (EventBus) or `ChainEvent` (Web3)
- `Action` can invoke an `Agent` (AI), update a `Project` (Business), send a `Transaction` (Web3)
- `Workflow` is installable as a `Module` (Platform)
- `Schedule` becomes a `Cron` primitive with mesh-aware execution (Network)

---

### 3. Web3 Domain

Everything needed to read, write, and transact on blockchains. **First-class, not optional.**

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Chain` | Blockchain network | Solana mainnet, devnet, Ethereum, Base |
| `Wallet` | Key material + accounts on a chain | Solana keypair, Ethereum EOA |
| `Key` | Cryptographic key (signing, encryption) | Ed25519, secp256k1 |
| `Account` | On-chain account/address | Token account, program account |
| `Token` | Fungible asset on a chain | SOL, USDC, SPL tokens, ERC-20 |
| `NFT` | Non-fungible asset | Module license, credential, badge |
| `Transaction` | Signed instruction set | Transfer, swap, stake, program call |
| `Program` / `Contract` | On-chain executable | Anchor program, Solidity contract |
| `Signature` | Cryptographic proof | Transaction sig, message sig |
| `Block` | Chain state checkpoint | Block number, timestamp, hash |
| `ChainEvent` | On-chain state change | Account update, transfer, program log |
| `DID` | Decentralized Identifier | did:key, did:sol, did:web |
| `Credential` | Verifiable Credential (W3C VC) | "Publisher of module X", "Team member" |
| `RPC` | Chain endpoint | Helius, Alchemy, local validator |

**Cross-domain links:**
- `Wallet.key` == `Identity.key` — same key material
- `ChainEvent` feeds `Trigger` (Automation) — on-chain events start workflows
- `Agent` reads `Account` state and builds `Transaction` (AI + Web3 fused)
- `Token` powers `Payment` in marketplace (Platform)
- `Credential` = `DID` + signed claim, verified by `TrustPort`
- `NFT` can represent a `Module` license

---

### 4. Business Domain

The actual work primitives that RUSVEL proved. These are what departments operate on.

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Project` | Container for related work | Client project, product sprint |
| `Task` | Unit of work with status | Todo, in progress, done, blocked |
| `Document` | Rich content with versioning | Proposal, spec, blog post |
| `Invoice` | Payment request | Amount, line items, due date, status |
| `Contact` | Person or organization | Client, vendor, team member |
| `Lead` | Potential business opportunity | Inbound inquiry, referral |
| `Pipeline` | Staged process | Sales pipeline, hiring funnel |
| `Campaign` | Marketing initiative | Email campaign, ad campaign |
| `Content` | Published media | Blog post, social post, newsletter |
| `Form` | Data collection interface | Intake form, survey, application |
| `Calendar` | Time-based scheduling | Events, availability, deadlines |
| `Ticket` | Support/service request | Bug report, feature request, question |
| `Account` (business) | Financial account for bookkeeping | Revenue, expenses, categories |
| `Report` | Aggregated view of data | Financial summary, pipeline status |

**Cross-domain links:**
- `Invoice` → on payment (`ChainEvent`), `Workflow` updates `Project` status
- `Agent` manages `Pipeline` — auto-qualifies `Lead`, creates `Task` for follow-up
- `Document` is an `Artifact` produced by `Agent`
- `Report` is generated by `Agent` reading `Store` + `ChainState`
- Every primitive has HLC + origin_node for CRDT sync

---

### 5. Platform Domain

The meta-layer: how rashk itself is extended, distributed, and governed.

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Module` | Installable capability unit | Rust app (Tier 1) or WASM (Tier 2) |
| `Manifest` | Module metadata + requirements | Name, version, ports needed, capabilities |
| `Registry` | Collection of available modules | Local installed, community marketplace |
| `Marketplace` | Public module discovery | Browse, rate, install, pay |
| `Rating` | Community quality signal | Stars, reviews, usage count |
| `License` | Usage rights for a module | Free, paid, subscription, NFT-gated |
| `Capability` | Permission grant to a module | "can read Store", "can send Transaction" |
| `Sandbox` | Isolation boundary (WASM) | Resource limits, allowed syscalls |
| `Version` | Semantic version of a module | 1.2.3, with migration path |
| `Hook` | Extension point in the runtime | Before/after events, middleware |

**Cross-domain links:**
- `Module` contains `Tool[]` (AI), `Workflow[]` (Automation), accesses `Chain` (Web3)
- `License` can be an `NFT` on-chain
- `Rating` feeds `TrustPort.reputation()`
- `Capability` governs what ports a `Module` can call
- `Marketplace` discovery via `Agent` — "find me an invoicing module"

---

### 6. Network Domain

Machine-to-machine connectivity, sync, and distribution.

| Primitive | Description | Examples |
|-----------|-------------|----------|
| `Node` | A rashk instance | My laptop, my desktop, team server |
| `Peer` | A connected node | Discovered via mDNS or mesh |
| `Mesh` | Network of connected nodes | Local LAN, WireGuard overlay, public |
| `Tunnel` | Encrypted connection between nodes | QUIC stream, WireGuard tunnel |
| `Sync` | Data reconciliation between nodes | CRDT merge, HLC-ordered |
| `Conflict` | Divergent state requiring resolution | Concurrent edits to same record |
| `Replica` | Copy of data on a node | Partial or full, with sync state |
| `Discovery` | Finding other nodes | mDNS (local), DHT (public), invite link |
| `Delegation` | Routing work to another node | "Run this agent on my desktop" |
| `Subscription` | Cross-node event subscription | "Notify me when desktop syncs" |

**Cross-domain links:**
- `Node` has `Identity` (Web3) — same keypair
- `Sync` uses `SyncPort` to merge `Record` CRDTs from `Store`
- `Delegation` sends `Agent` work to a `Peer` via `MeshPort`
- `Discovery` feeds into `Mesh` topology
- Every `Record` carries HLC + `Node` origin for conflict resolution

---

## The Universal Primitive Protocol

Every primitive in every domain follows the same contract. This is what makes rashk a platform, not an app.

```
trait Primitive: Serialize + Deserialize + Send + Sync {
    type Id;

    // Identity
    fn id(&self) -> &Self::Id;
    fn kind(&self) -> &str;         // "ai.agent", "web3.token", "biz.invoice"
    fn version(&self) -> u64;       // HLC for CRDT

    // Metadata
    fn origin_node(&self) -> &NodeId;
    fn created_at(&self) -> HlcTimestamp;
    fn updated_at(&self) -> HlcTimestamp;
    fn metadata(&self) -> &serde_json::Value;
}
```

And every primitive gets these operations for free through the port layer:

| Operation | How | Port |
|-----------|-----|------|
| **Store** | `store.put(record)` / `store.get(id)` | `Store` |
| **Search** | `search.search("unpaid invoices")` | `SearchIndex` |
| **Semantic search** | `vector.search_similar(embedding)` | `VectorStore` |
| **Events** | `bus.publish(Event::Created(invoice))` | `EventBus` |
| **Sync** | Automatic via HLC + origin_node | `SyncPort` |
| **Mesh relay** | Transparent via adapter | `MeshPort` |
| **AI access** | Agent reads/writes primitives as tools | `AgentPort` |
| **Chain anchoring** | Hash on-chain for provenance | `ChainPort` (new) |

---

## How Primitives Map to Ports

The 6 domains produce ~80 primitive types. But they all flow through the same small set of ports:

```
                              ┌─────────────────┐
                              │  80+ Primitives  │
                              │  across 6 domains│
                              └────────┬─────────┘
                                       │
                    ┌──────────────────────────────────────┐
                    │        Universal Primitive Protocol    │
                    │  (Serialize, Id, Kind, HLC, Origin)   │
                    └──────────────────┬───────────────────┘
                                       │
        ┌──────────┬──────────┬────────┼────────┬──────────┬──────────┐
        │          │          │        │        │          │          │
   ┌────▼───┐ ┌───▼───┐ ┌───▼───┐ ┌──▼──┐ ┌──▼───┐ ┌───▼───┐ ┌───▼────┐
   │ Store  │ │Search │ │Vector │ │Event│ │Sync  │ │Agent  │ │ Chain  │
   │        │ │Index  │ │Store  │ │Bus  │ │Port  │ │Port   │ │ Port   │
   └────────┘ └───────┘ └───────┘ └─────┘ └──────┘ └───────┘ └────────┘
        │          │          │        │        │          │          │
   ┌────▼──────────▼──────────▼────────▼────────▼──────────▼──────────▼───┐
   │                        Adapter Layer                                  │
   │  SQLite │ FTS5 │ LanceDB │ InMem │ cr-sqlite │ Claude │ Solana RPC  │
   └──────────────────────────────────────────────────────────────────────┘
```

The primitives are MANY. The ports are FEW. That's the leverage.

---

## How This Enables "Extend Beyond Imagination"

### Community writes new primitives, not new infrastructure

Someone wants a `Proposal` primitive for DAO governance? They write a Rust app module or WASM module that:
1. Defines the `Proposal` struct (implements `Serialize + Deserialize`)
2. Registers tools: `governance.create_proposal`, `governance.vote`
3. Emits events: `proposal.created`, `proposal.passed`
4. Uses existing ports: `Store` (persistence), `ChainPort` (on-chain voting), `Agent` (summarize proposal)

They don't build storage, search, sync, AI, or blockchain integration. They get all of it for free from the port layer.

### Open-source wrapping becomes mechanical

Wrapping n8n's workflow engine:
1. n8n's `Flow`, `Node`, `Trigger`, `Action` map to rashk's Automation primitives
2. n8n's execution engine becomes a Rust app module
3. n8n's data model maps to `Store` records
4. n8n's webhook triggers map to rashk `Trigger` primitives
5. The AI, Web3, and mesh layers are **added for free** — n8n never had them

Same pattern for Plane (→ Business primitives), Supabase (→ Storage primitives), Invoice Ninja (→ Business + Web3 primitives).

### The marketplace sells primitives, not apps

A module on the marketplace isn't "an invoicing app." It's:
- 4 primitive types: `Invoice`, `LineItem`, `PaymentRequest`, `Receipt`
- 6 tools: `invoice.create`, `invoice.send`, `invoice.mark_paid`, ...
- 3 workflows: "auto-send reminder", "on-chain payment detection", "monthly summary"
- 2 agent skills: "generate invoice from conversation", "reconcile payments"

Another module can compose these primitives. An accounting module references `Invoice` by ID. A reporting module aggregates `Invoice` + `Transaction`. Primitives compose across module boundaries.

---

## Port Evolution: What Changes

Based on this model, the port list needs adjustment:

### Add
| Port | Why |
|------|-----|
| `ChainPort` | Read on-chain state: accounts, tokens, transactions, events. Current PaymentPort is too thin. |
| `TransactionPort` | Build, simulate, sign, submit chain transactions. Separated from reading. |
| `ChainListener` | Subscribe to on-chain events. Feeds EventBus. |

### Evolve
| Port | Change |
|------|--------|
| `PaymentPort` | Becomes a Rust app module composing ChainPort + TransactionPort. Not a core port. |
| `MeshPort` + `MeshReceiver` | Merge into single `MeshPort` with send/receive/subscribe. |
| `ModuleSource` | Defer to Phase 5. Not needed for Tier 1 (Rust app modules). |
| `LlmPort` | Expand: add embeddings, streaming, tool-use mode, model listing. |
| `AgentPort` | Add: memory context, artifact output, skill reference. |

### Keep as-is
Store, BlobStore, SearchIndex, VectorStore, EventBus, SyncPort, ModuleRegistry, ModuleRuntime, IdentityPort, TrustPort, CapabilityPort.

---

## The Primitive Registry

Every domain's primitives register into a unified registry at boot time:

```
PrimitiveRegistry
├── ai.provider      → [openai, anthropic, ollama]
├── ai.agent         → [finance-agent, code-agent]
├── ai.tool          → [store.query, chain.balance, ...]
├── ai.skill         → [analyze-codebase, generate-invoice]
├── auto.workflow     → [on-payment-update-project, ...]
├── auto.trigger      → [cron:daily-9am, event:invoice.paid]
├── web3.chain        → [solana-mainnet, solana-devnet]
├── web3.wallet       → [main-wallet, trading-wallet]
├── web3.token        → [SOL, USDC, RASHK]
├── biz.project       → [client-alpha, rashk-dev]
├── biz.invoice       → [INV-001, INV-002]
├── platform.module   → [dept-finance, dept-code, ...]
├── net.node          → [my-laptop, my-desktop]
└── ...
```

Agents can query this: "list all web3.token primitives" or "find biz.invoice where status = unpaid". The registry IS the Store — primitives are just typed records with a `kind` prefix.

---

## What This Document Is

This is the **constitution of rashk as a platform**. The port traits define HOW things work. This domain model defines WHAT things exist. Together they answer:

- Ports: "How do I store, search, sync, communicate, transact?"
- Primitives: "What are the nouns of the system? What can I build with?"

Every future ADR, every module design, every API surface should reference this document to check: "Is this primitive first-class? Does it follow the universal protocol? Does it compose with the rest?"

---

*Created: 2026-03-29*
*Status: Draft — to be refined as ADRs land*
