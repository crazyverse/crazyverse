# RASHK — Full Stack Architecture

> From OS to marketplace. Every layer purpose-built for the Universe Model.
> Not building from scratch — configuring, forking, composing what exists.

## The Insight

ChromeOS is Linux + Chrome. Brave is Chromium + crypto wallet. Tailscale is WireGuard + coordination. None of them rewrote the stack — they **composed existing open-source at every layer** and added the missing piece that makes the whole thing cohere.

RASHK does the same. The missing piece is: **an agentic, contractual, composable runtime that connects entities, services, and value exchange across every layer of the stack.**

---

## The Full Stack

```
┌──────────────────────────────────────────────────────────────────────┐
│ Layer 7: MARKETPLACE                                                  │
│   Module registry, service directory, learning platform               │
│   Anyone publishes. Anyone discovers. Contracts on-chain.             │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 6: APPLICATIONS                                                 │
│   Business modules (CRM, invoicing, PM, chat, CMS...)                │
│   Rust app modules (Tier 1) + WASM modules (Tier 2)                  │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 5: SERVICES                                                     │
│   Communication (Telegram-style) │ Learning │ Building │ Commerce     │
│   Each is a composable service on the runtime, not a separate app     │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 4: RUNTIME (rashk binary — the core product)                    │
│   Ports │ Adapters │ Agents │ Workflows │ Events │ CRDT Sync         │
│   80+ domain primitives across 6 domains                              │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 3: WEB3 + MESH                                                  │
│   Solana (contracts, payments) │ TON (Telegram distribution)          │
│   Tailscale-model mesh │ CRDT sync │ P2P discovery                    │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 2: BROWSER (RashkBrowser — Chromium fork)                       │
│   WASM-native │ Built-in wallet │ Built-in identity │ Agent sidebar   │
│   MCP tools │ P2P WebRTC │ Offline-first │ Telegram Mini App host     │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 1: OS (RashkOS — purpose-built Linux)                           │
│   Immutable │ Mesh-ready │ WASM sandbox │ Auto-update │ Minimal       │
│   Runs on: laptop, desktop, Raspberry Pi, VPS, cloud VM              │
└──────────────────────────────────────────────────────────────────────┘
```

Every layer builds on the one below. Every layer is independently useful. You can use Layer 4 (runtime) without Layer 1 (OS). You can use Layer 2 (browser) without Layer 6 (apps). But together, they form a complete sovereign computing stack.

---

## Layer 1: RashkOS

**What:** A purpose-built Linux distribution optimized for running RASHK nodes.
**Not:** Rewriting Ubuntu from scratch. Fork + configure + strip.

### Why an OS?

Because the Universe Model needs machines that are:
- **Always-on mesh nodes** — not apps you launch and close
- **Self-updating** — immutable OS, atomic updates, rollback
- **Minimal attack surface** — no desktop bloat, only what RASHK needs
- **Reproducible** — same OS image on a laptop, a Pi, a VPS
- **Pre-configured** — mesh networking, WASM runtime, wallet, identity — all built in

### Base: NixOS (immutable, reproducible, declarative)

| Why NixOS | Not Ubuntu/Debian |
|-----------|------------------|
| Immutable system (no drift) | Mutable, config drift over time |
| Declarative config (one file = entire OS) | Imperative (apt install, manual config) |
| Atomic rollback | No clean rollback |
| Reproducible builds | "Works on my machine" |
| Flakes for version pinning | Version conflicts common |
| Cross-compilation built in | Cross-compilation is painful |
| Small community but growing fast | Huge community but legacy patterns |

**Alternative considered:** Flatcar Linux (container-optimized, immutable) — simpler but less flexible than NixOS for custom services.

### What ships in RashkOS

```
rashk-os/
├── kernel           ← Linux kernel (minimal config, stripped modules)
├── init             ← systemd with rashk.service pre-configured
├── rashk            ← The rashk binary (Layer 4 runtime)
├── rashk-browser    ← Chromium fork (Layer 2)
├── tailscale        ← Mesh networking daemon (or headscale client)
├── wasmtime         ← WASM runtime (linked into rashk)
├── sqlite           ← Local-first data (linked into rashk)
├── wireguard        ← Kernel module for encrypted tunnels
├── avahi            ← mDNS for local peer discovery
├── rashk-updater    ← Atomic OS updates (nix-based)
└── rashk-config.nix ← Single declarative config file
```

### Configuration: one file

```nix
# /etc/rashk/config.nix — the entire node configuration
{
  rashk = {
    node.name = "mehdi-laptop";
    identity.keyfile = "/var/rashk/identity.key";

    mesh = {
      enable = true;
      coordinator = "headscale.myteam.dev";  # or "auto" for mDNS-only
      relay.enable = true;  # this node can relay for others
    };

    chains = {
      solana = {
        rpc = "https://api.mainnet-beta.solana.com";
        wallet = "/var/rashk/solana-keypair.json";
      };
      ton = {
        enable = true;
        liteserver = "auto";
      };
    };

    modules.autoInstall = [ "finance" "code" "content" ];

    browser.enable = true;
    browser.defaultPage = "rashk://dashboard";
  };
}
```

### Image targets

| Target | Form factor | Use case |
|--------|-------------|----------|
| `rashk-os-desktop.iso` | Bootable USB / install | Primary workstation |
| `rashk-os-server.img` | Headless, no browser | VPS / always-on node |
| `rashk-os-pi.img` | ARM, Raspberry Pi | Home server / mesh relay |
| `rashk-os-vm.qcow2` | QEMU/KVM/cloud | Cloud VM deployment |
| `rashk-os-docker.tar` | Container | Existing infrastructure |
| `rashk-os-wsl.tar` | WSL2 | Windows users |

### Phase: 8+ (after core runtime is proven)

RashkOS is the final packaging layer. It doesn't block anything — RASHK runs on any Linux/macOS/Windows today. The OS layer is about **zero-config deployment** and **fleet management** for the community/enterprise scale.

---

## Layer 2: RashkBrowser

**What:** A Chromium-based browser with RASHK's universe model built into the browsing experience.
**Not:** Rewriting a rendering engine. Fork Chromium (like Brave, Arc, Vivaldi all do).

### Why a browser?

Because in the Universe Model, the browser IS the universal client:
- **Every entity has a presence** — browsable, not just API-accessible
- **Agents live in the browser** — sidebar agent with full tool access
- **Wallet is native** — no extension needed (like Brave's built-in wallet)
- **Identity is native** — DID/keypair managed by the browser, not a website
- **WASM runs natively** — RASHK modules execute in-browser via jco
- **Mesh participates** — browser is a node via WebRTC
- **Offline works** — local-first, service workers, IndexedDB/OPFS + cr-sqlite

### What Chromium already gives us

| Feature | Chromium built-in | RASHK adds |
|---------|-------------------|------------|
| Rendering | Blink engine | Nothing to change |
| JavaScript | V8 | Agent runtime scripts |
| WASM | V8 WASM support | RASHK module loader (via jco) |
| WebRTC | Built-in | Mesh peer connections |
| Service Workers | Built-in | Offline-first sync |
| IndexedDB / OPFS | Built-in | cr-sqlite CRDT store |
| DevTools | Built-in | Agent debugging tools |
| Extensions API | Manifest V3 | RASHK module bridge |

### What RASHK adds to Chromium

```
rashk-browser/
├── chromium/                  ← Upstream Chromium source
├── rashk-shell/               ← Custom new tab / dashboard (SvelteKit)
├── rashk-sidebar/             ← Agent sidebar (always available)
│   ├── agent-chat.svelte      ← Talk to your agent
│   ├── tool-panel.svelte      ← Available tools from modules
│   └── wallet-panel.svelte    ← Balance, transactions, sign
├── rashk-wallet/              ← Built-in multi-chain wallet
│   ├── solana-adapter.rs      ← Solana keypair management
│   ├── ton-adapter.rs         ← TON keypair + TON Connect
│   └── signing-ui.svelte      ← Transaction approval prompts
├── rashk-identity/            ← Built-in DID management
│   ├── keystore.rs            ← Secure key storage (OS keychain)
│   ├── did-resolver.rs        ← did:key, did:sol, did:web resolution
│   └── credential-ui.svelte   ← Verifiable credential viewer
├── rashk-mesh/                ← Browser as mesh node
│   ├── webrtc-adapter.rs      ← P2P via WebRTC data channels
│   ├── sync-worker.js         ← CRDT sync in service worker
│   └── discovery.rs           ← Find peers via signaling server
├── rashk-modules/             ← WASM module runtime in browser
│   ├── jco-loader.js          ← Load WASM components via jco
│   ├── capability-sandbox.js  ← Enforce capability grants
│   └── module-bridge.js       ← Connect modules to browser APIs
├── rashk-protocols/           ← Custom protocol handlers
│   ├── rashk://               ← rashk://dashboard, rashk://module/finance
│   ├── entity://              ← entity://did:sol:abc123 → entity profile
│   └── service://             ← service://invoice/create → service UI
└── rashk-devtools/            ← Developer tools panel
    ├── agent-debugger.svelte  ← Agent execution trace viewer
    ├── mesh-inspector.svelte  ← Connected peers, sync state
    └── chain-explorer.svelte  ← Transaction viewer
```

### The `rashk://` protocol

The browser registers custom protocol handlers:

| URL | Resolves to |
|-----|-------------|
| `rashk://dashboard` | Home dashboard — modules, mesh status, wallet |
| `rashk://modules` | Installed modules, marketplace browser |
| `rashk://agent` | Full-page agent interface |
| `rashk://mesh` | Connected peers, sync status |
| `rashk://wallet` | Multi-chain wallet view |
| `entity://did:sol:abc123` | Entity profile page (person, business, agent) |
| `service://rashk:invoice/create` | Service invocation UI |
| `module://finance/ledger` | Module-specific UI |

### Telegram Mini App mode

RashkBrowser can run as a Telegram Mini App host:
- SvelteKit frontend is the Mini App
- TON Connect provides wallet integration
- Same codebase, different shell (Telegram TWA vs browser chrome)
- 500M Telegram users get RASHK without installing a browser

### Phase: 7-8 (after runtime + modules + mesh are proven)

The browser is a distribution vehicle, not a technical dependency. Everything works via the SvelteKit web app in any browser first. The custom browser adds: native wallet, native identity, native agent sidebar, native mesh participation, custom protocols.

---

## Layer 3: Web3 + Mesh

Already designed in detail. See: `open-source-mapping.md` (Solana, TON, Tailscale sections).

### Summary

| Component | Source | RASHK layer |
|-----------|--------|-------------|
| Smart contracts / escrow | Solana (Anchor, Rust) | Contract enforcement |
| Payments / tokens | Solana SPL tokens | Value transfer |
| Telegram distribution | TON Connect + Mini Apps | 500M user reach |
| .ton domain discovery | TON DNS | Entity naming |
| Mesh coordination | Headscale → boringtun | Node connectivity |
| NAT traversal | STUN → Peer Relay → DERP | Universal reachability |
| Local discovery | mDNS / Avahi | Zero-config LAN mesh |
| Encrypted tunnels | WireGuard (kernel) | Data plane security |

---

## Layer 4: Runtime (the core — already designed)

See: `MASTER_PLAN.md`, `domain-model.md`, `universe.md`.

This is the rashk binary. Port traits, adapters, domain primitives, agent loop, event bus, CRDT sync. Everything below supports it. Everything above runs on it.

---

## Layer 5: Services

Services are NOT separate apps. They are **compositions of modules + primitives + workflows** running on the Layer 4 runtime.

### Communication Service (Telegram-inspired)

**What Telegram got right:** Instant messaging as a platform (bots, channels, Mini Apps, payments, groups).

**RASHK equivalent:**

| Telegram concept | RASHK implementation |
|-----------------|---------------------|
| Messages | `biz.message` primitive in Store, E2E encrypted via IdentityPort |
| Channels | `EventBus` topic subscriptions with capability-gated read |
| Groups | Mesh sub-networks with shared Store + CRDT sync |
| Bots | Agents with tool registrations (MCP-compatible) |
| Mini Apps | WASM modules with SvelteKit UI fragments |
| Payments | ChainPort + TransactionPort (Solana/TON) |
| Voice/Video | WebRTC via MeshPort (P2P, encrypted) |
| Stories/Media | BlobStore + content module |

Not a Telegram clone. A **communication primitive layer** that any module can use:

```rust
// Any module can send messages through the communication service
comm.send(Message {
    to: entity_did,
    content: MessageContent::Text("Invoice INV-042 is ready".into()),
    channel: "notifications",
    actions: vec![
        Action::button("View Invoice", "service://invoice/INV-042"),
        Action::button("Pay Now", "service://payment/INV-042"),
    ],
});
```

### Learning Service

**The platform for anyone in the world to learn, build, and participate.**

| Component | Implementation |
|-----------|---------------|
| Courses | `biz.course` → `biz.lesson` → `biz.exercise` primitives in Store |
| Interactive coding | Agent-assisted (Claude Code patterns) + WASM sandbox |
| Progress tracking | Store records per learner, synced via CRDT |
| Certification | Verifiable Credentials (W3C VC) issued by IdentityPort, anchored on-chain |
| Peer review | Agent + human hybrid, reputation-scored |
| Monetization | Course creator publishes → learner pays via ChainPort (on-chain, transparent) |
| Localization | Agent-powered translation, community-reviewed |

The learning service teaches people to:
1. **Use** RASHK (as an end user)
2. **Build** modules (as a developer — Rust app or WASM)
3. **Offer** services (as a freelancer/business)
4. **Participate** in governance (as a community member)

### Building Service (Claude Code-inspired)

**An AI-powered development environment for creating modules, services, and businesses on RASHK.**

| Component | Implementation |
|-----------|---------------|
| Code editor | Monaco (VS Code engine) in SvelteKit, or full VS Code via code-server |
| AI coding agent | AgentPort with code tools (file read/write/edit, terminal, search) |
| Module scaffolding | Agent generates Rust app or WASM module from natural language |
| Testing sandbox | wasmtime instance for module testing, mock ports |
| Publishing | Module → signed → pushed to registry → discoverable |
| Self-healing | Agent monitors module health, auto-fixes, auto-deploys |
| Collaboration | CRDT sync of code across mesh (like Zed editor) |

```
User: "I want to build an invoicing module"

Agent: Creates rashk-module-invoice/
  ├── Cargo.toml (rashk-core dependency)
  ├── src/lib.rs (Invoice, LineItem, PaymentRequest primitives)
  ├── src/manifest.rs (module manifest with tools + capabilities)
  ├── src/tools.rs (invoice.create, invoice.send, invoice.mark_paid)
  ├── src/workflows.rs (auto-reminder, payment-detection)
  ├── wit/invoice.wit (WIT interface for WASM build)
  ├── ui/ (SvelteKit components for invoice views)
  └── tests/

Agent: Builds, tests, signs, publishes to registry
User: Earning money as others install and use the module
```

### Commerce Service

**The marketplace where entities trade.**

| Component | Implementation |
|-----------|---------------|
| Service listings | `platform.listing` primitives in Store, indexed by SearchIndex |
| Entity profiles | `entity://did:sol:...` pages with reputation, services, portfolio |
| Discovery | Agent-assisted search ("find me a Rust developer under $100/hr with >90% rep") |
| Contracting | On-chain escrow via Solana Anchor programs |
| Payment | SPL tokens (Solana) or Jettons (TON) |
| Ratings/Reviews | On-chain reputation SBTs, verified by TrustPort |
| Dispute resolution | Multi-sig arbitration contract |
| Categories | Module-defined (any module can register a service category) |

### Service composition

Services are NOT silos. They compose through primitives:

```
Learning Service                    Commerce Service
    │                                    │
    ├─ Learner completes course          │
    ├─ IdentityPort issues VC            │
    │  (on-chain credential)             │
    ├─ Credential visible on             │
    │  entity://did:sol:learner          │
    │                                    │
    │        ┌───────────────────────────┤
    │        │                           │
    │        ▼                           │
    │  Learner lists service             │
    │  ("Rust module development")       │
    │  backed by verified credential     │
    │                                    │
    │  Buyer discovers via agent ────────┤
    │  Contract on-chain ────────────────┤
    │  Agent executes work ──────────────┤
    │  Payment auto-released ────────────┤
    │  Reputation updated ───────────────┘
```

One flow crosses Learning → Commerce → Identity → Chain → Agent → Payment. No glue code. Just primitives composing through ports.

---

## Layer 6: Applications

Rust app modules (Tier 1, compiled-in) and WASM modules (Tier 2, sandboxed):

| Category | Modules | Inspired by |
|----------|---------|-------------|
| Project Management | Tasks, sprints, boards | Plane, Vikunja |
| CRM | Contacts, leads, pipelines | EspoCRM |
| Invoicing | Invoices, payments, receipts | Invoice Ninja |
| Workflow Automation | DAGs, triggers, actions | n8n |
| Content Management | Posts, pages, media | Ghost, Strapi |
| Team Chat | Messages, channels, threads | Mattermost |
| Scheduling | Calendar, bookings, availability | Cal.com |
| Support | Tickets, knowledge base | Chatwoot |
| Forms | Surveys, intake, applications | Formbricks |
| Accounting | Ledger, reports, tax | Akaunting |
| Data Views | Grid, kanban, gallery, calendar | NocoDB |
| Code | Repository, CI/CD, review | Gitea |
| DeFi | Portfolio, swaps, staking | Custom |
| DAO Governance | Proposals, voting, treasury | SPL Governance |

Every module gets AI, automation, blockchain, and mesh **for free** through the port layer.

---

## Layer 7: Marketplace

The self-sustaining flywheel:

```
BUILDER                          USER
  │                                │
  ├─ Builds module                 │
  ├─ Signs with keypair            │
  ├─ Publishes to registry         │
  │                                │
  │     ┌── MARKETPLACE ──┐        │
  │     │                  │        │
  │     │  Agent-assisted  │◄───────┤ Discovers
  │     │  discovery       │        │
  │     │                  │        │
  │     │  On-chain        │        │
  │     │  reputation      │        │
  │     │                  │        │
  │     │  Smart contract  │◄───────┤ Contracts
  │     │  licensing       │        │
  │     │                  │        │
  │     └──────────────────┘        │
  │                                │
  ◄────── SPL tokens ──────────────┤ Pays
  │                                │
  ├─ Reputation grows              │
  ├─ More visibility               │
  ├─ More income                   │
  └─ Builds more modules           │
```

### Revenue model (for the ecosystem, not a company)

| Revenue stream | How | Who benefits |
|---------------|-----|-------------|
| Module sales | One-time or subscription, on-chain | Module builders |
| Service fees | Per-use micropayments | Service providers |
| Agent work | Agent-to-agent transactions | Entity owners |
| Learning courses | Course creators set prices | Educators |
| Staking | Stake tokens for priority in registry | Long-term holders |
| Governance | Token-weighted voting on standards | Community |
| Network fee | Minimal % on marketplace transactions | Protocol treasury (DAO) |

No company takes 30%. The protocol treasury is governed by token holders. The fee exists only to sustain infrastructure (relay nodes, registry hosting, chain fees).

---

## The Participation Path

How anyone in the world participates:

### Path 1: User (day one)

```
1. Install rashk (single binary, any OS)
   OR visit rashk in browser
   OR open Telegram Mini App
2. Create identity (keypair generated locally)
3. Install modules you need (invoicing, PM, CRM...)
4. Work — agents assist, data is local, syncs across your devices
```

### Path 2: Builder (week one)

```
1. Use rashk as a user first
2. Open Building Service (AI-assisted dev environment)
3. Describe what you want to build in natural language
4. Agent scaffolds a module, helps you code, tests, iterates
5. Sign and publish to marketplace
6. Earn when others install your module
```

### Path 3: Service Provider (month one)

```
1. Register as entity (DID, keypair, profile)
2. List your services (skills, portfolio, rates)
3. Agent handles discovery, negotiation, scheduling
4. Deliver work → verified on-chain → payment auto-released
5. Reputation accumulates → more visibility → more work
```

### Path 4: Educator (ongoing)

```
1. Create courses on the Learning Service
2. Interactive: coding exercises run in WASM sandbox
3. Certification: completions issued as Verifiable Credentials
4. Students pay per-course or subscription
5. Credentials are on-chain — verifiable by anyone, forever
```

### Path 5: Community Governor (as tokens accumulate)

```
1. Hold RASHK tokens (earned or purchased)
2. Vote on module standards, registry policies, protocol upgrades
3. Propose changes via DAO governance
4. Stake for registry priority
5. Fund grants for needed modules
```

---

## Build Order (Revised Phases)

The original 8 phases still hold, but with the full stack in mind:

| Phase | Focus | Layers touched |
|-------|-------|----------------|
| **0** | Design & Research (NOW) | All (design only) |
| **1** | Runtime + one module | Layer 4 |
| **2** | CRDT Sync | Layer 4 |
| **3** | Mesh Networking | Layer 3 |
| **4** | Identity & Chain | Layer 3 + 4 |
| **5** | Module Registry + Marketplace | Layer 6 + 7 |
| **6** | RUSVEL Migration | Layer 6 |
| **7** | Services (Communication, Learning, Building, Commerce) | Layer 5 |
| **8** | RashkBrowser (Chromium fork) | Layer 2 |
| **9** | RashkOS (NixOS-based distro) | Layer 1 |
| **10** | Open-source wrapping (n8n, Supabase, NocoDB patterns as modules) | Layer 6 |

Layers 1 and 2 come LAST because they're packaging — the runtime works without them. But designing with them in mind from Phase 0 ensures nothing we build blocks them later.

---

## How Each Open-Source Project Slots In

| Project | Layer | Role |
|---------|-------|------|
| **Linux / NixOS** | 1 (OS) | Base OS, immutable, declarative config |
| **Chromium** | 2 (Browser) | Rendering engine, WASM support, WebRTC |
| **Tailscale / Headscale** | 3 (Mesh) | Coordination server, NAT traversal, WireGuard |
| **Solana** | 3 (Web3) | Smart contracts, payments, identity, governance |
| **TON** | 3 (Web3) | Telegram distribution, .ton discovery, Jettons |
| **wasmtime** | 4 (Runtime) | WASM module execution, WIT bindings |
| **cr-sqlite** | 4 (Runtime) | CRDT-enabled SQLite for local-first sync |
| **SvelteKit** | 4 (Runtime) | Universal UI framework (browser, Mini App, desktop) |
| **n8n** | 4+5 (Runtime+Services) | Workflow patterns, DAG format, node registry |
| **Google ADK** | 4+5 (Runtime+Services) | Agent architecture, event streams, tool discovery |
| **Claude Code** | 4+5 (Runtime+Services) | MCP protocol, skills, hooks, AI coding agent |
| **Supabase** | 4 (Runtime) | RLS→CapabilityPort, WIT WASM pattern, CDC→EventBus |
| **NocoDB** | 6 (Apps) | Metadata-over-data, schema-as-UI, view abstraction |

---

## The One Sentence

**RASHK is a sovereign computing stack — from OS to marketplace — where every entity, service, and transaction is a first-class primitive that agents can discover, compose, and execute across a decentralized mesh, with contracts on-chain and payments in code.**

---

*Created: 2026-03-29*
*This document extends: universe.md (vision), domain-model.md (primitives), open-source-mapping.md (technology)*
