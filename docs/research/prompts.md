# Research Prompts — Perplexity

> Feed each prompt to Perplexity. Save responses as `P{N}-{slug}.md` in this directory.
> Status: PENDING = not yet researched, DONE = response saved

## Critical (blocks Phase 1 design)

### P1: WASM Component Model in Production — PENDING
```
What production systems use the WebAssembly Component Model with WIT
interfaces as of March 2026? Specifically looking for:
- Rust hosts using wasmtime's bindgen! macro with custom WIT interfaces
- Supabase Wrappers (postgres-wasm-fdw) architecture and lessons learned
- Fermyon Spin, Cosmonic, or other platforms using WASI 0.2 components
- Real performance numbers: host-guest call latency, memory overhead
- Any known limitations or gotchas when defining custom WIT interfaces
  (not just using WASI standard interfaces)
Focus on Rust host implementations, not browser or JavaScript.
```

### P2: Solana Escrow & Service Marketplace Programs — PENDING
```
What existing Solana programs (Anchor/Rust) implement escrow,
service marketplace, or freelance-style contracts as of 2026?
Looking for:
- Open-source Anchor programs for escrow with milestone-based release
- Service marketplace contracts (list service, accept, deliver, pay)
- On-chain reputation/review systems (SBTs or similar)
- How PDAs (Program Derived Addresses) are used for entity/service registration
- Real gas costs for a typical escrow flow (create + fund + release)
- Any production examples of agent-initiated transactions (not human wallet signing)
Compare with TON's async message model for the same escrow pattern.
```

### P3: cr-sqlite Production Readiness — DONE
```
What is the production status of cr-sqlite (Conflict-free Replicated
SQLite) as of March 2026? Specifically:
- Latest version, who maintains it, funding status
- What conflict types does it handle? (concurrent inserts, updates, deletes)
- Schema migration story — can you ALTER TABLE on a CRDT-enabled db?
- Real-world deployments and scale (how many nodes, how much data?)
- Comparison with Automerge 2.0 and Electric SQL for structured data sync
- SQLite WASM compatibility — does cr-sqlite work in browser via OPFS?
- Rust integration — are there Rust bindings or is it C-only?
Focus on structured business data (invoices, tasks, contacts), not
collaborative text editing.
```

### P4: Headscale + WireGuard Mesh in Practice — DONE
```
How are teams using Headscale (open-source Tailscale coordination
server) in production as of 2026? Specifically:
- Largest known deployments (number of nodes)
- NAT traversal success rate in practice (what % of connections go direct vs DERP relay?)
- Self-hosted DERP relay setup — how complex?
- Peer Relay feature (GA Feb 2026) — does Headscale support it?
- boringtun (Cloudflare pure-Rust WireGuard) — current maturity, latest release, any production use?
- Anyone embedding WireGuard in a Rust application binary (not as OS daemon)?
- NetBird, Nebula, or ZeroTier as alternatives to the Headscale model?
Focus on self-hosted, no-vendor-dependency mesh networking.
```

## High Priority (shapes architecture)

### P5: Telegram Mini Apps as Distribution — DONE
```
What are the most successful Telegram Mini Apps built on TON as of
2026? Looking for:
- Top Mini Apps by MAU (monthly active users) and revenue
- Technical stack — what frameworks are they built with? (React, Vue, Svelte?)
- TON Connect integration patterns — best practices for wallet connection
- Can a Mini App run as a full Progressive Web App outside Telegram too?
- SvelteKit specifically — any Telegram Mini Apps built with SvelteKit?
- Revenue models that work (in-app purchases via Jettons, subscriptions, tips)
- Approval process and restrictions from Telegram
- Real conversion rates: Telegram users → Mini App active users
Focus on apps that involve payments/transactions, not just games.
```

### P6: Agent-to-Agent Commerce / Autonomous Agent Economy — DONE
```
Are there any production or research systems where AI agents
autonomously discover services, negotiate terms, and execute
financial transactions as of 2026? Looking for:
- Google A2A protocol (Agent-to-Agent) — any real deployments beyond demos?
- Fetch.ai, Autonolas, or other "agent economy" projects — actual traction?
- MCP (Model Context Protocol) used for agent-to-service discovery
- Agents that hold crypto wallets and initiate transactions autonomously
- Smart contracts designed for agent (not human) interaction
- Legal/regulatory considerations for autonomous agent payments
- Academic research on agent marketplaces or service negotiation protocols
This is the core thesis: agents discover, contract, execute, and pay
— without human intervention for each transaction.
```

### P7: Multi-Chain Wallet Architecture in Rust — DONE
```
How do multi-chain wallets (Phantom, Backpack, Brave Wallet)
architect their key management across Solana + EVM + other chains?
- Shared seed phrase → chain-specific derivation paths
- Ed25519 (Solana) vs secp256k1 (Ethereum/TON) from same master key
- TON key format compatibility with Solana Ed25519 keys
- Rust crates for multi-chain key derivation (ed25519-dalek, bip39, etc.)
- Secure key storage patterns (OS keychain, hardware security modules)
- How Brave Wallet implemented built-in multi-chain in Chromium
Focus on Rust implementations and key material sharing between
Solana and TON specifically.
```

### P8: Chromium Fork Complexity — DONE
```
What does it take to maintain a Chromium fork in 2026? Looking at
Brave, Arc, Vivaldi, and other Chromium-based browsers:
- Team size dedicated to maintaining the Chromium fork
- How often do they rebase on upstream Chromium? How painful is it?
- Brave's approach to adding crypto wallet, BAT token, IPFS natively
- Arc's approach to adding AI features into the browser chrome
- Custom protocol handlers (brave://, arc://) — how are they registered?
- Build time and infrastructure requirements for Chromium
- Is there a minimal Chromium embedding approach (CEF, electron-less)?
- WebView-based alternatives (Tauri with system WebView) vs full fork
Honest assessment: is a Chromium fork feasible for a small team (1-5),
or does it require 20+ engineers?
```

## Medium Priority (informs decisions)

### P9: NixOS as Appliance OS — DONE
```
Who is using NixOS to build appliance/purpose-built OS images in 2026?
- NixOS generators for ISO/VM/container images
- Comparison with Flatcar Linux, Talos, or Bottlerocket for immutable OS
- NixOS on Raspberry Pi — current support, image size, boot time
- Fleet management with NixOS (nixops, colmena, deploy-rs)
- Declarative full-system config → reproducible across hardware
- Any projects using NixOS as the base for a product (not just dev env)?
Focus on minimal, single-purpose OS images that auto-update.
```

### P10: Module Marketplace Economics — DONE
```
What are the economics of software module/plugin marketplaces?
- npm: how much do package authors earn? (answer: nothing)
- Shopify App Store: revenue split, average developer income
- WordPress plugin marketplace: revenue models that work
- Figma plugins: how they monetize
- Docker Hub / Artifact Hub: distribution without monetization
- Any marketplace using cryptocurrency for payments?
- What marketplace features drive developer adoption vs user discovery?
- Typical commission rates and what marketplace operators provide for it
Looking for data to design a module marketplace where builders actually
earn meaningful income, with on-chain transparent payment splits.
```

### P11: DID Methods in Practice — DONE
```
Which Decentralized Identifier (DID) methods have real adoption
as of 2026?
- did:key — simplest, any production use beyond demos?
- did:web — DNS-based, who's using it in production?
- did:sol — Solana-based, any active implementations?
- did:ion — Bitcoin-based (Microsoft), status after ION shutdown rumors
- did:pkh — wallet-address-based, bridge between Web3 and DID world
- Verifiable Credentials (W3C VC) — which Rust libraries can issue/verify?
- SpruceID, DIF, or other DID ecosystem tools with Rust support
- Real-world adoption numbers: how many DIDs exist on each method?
Focus on methods that work with Ed25519 keys (Solana-compatible).
```

---

## Decision Mapping

| Prompt | Informs Decision | MASTER_PLAN ref |
|--------|-----------------|-----------------|
| P1 | D1 (WASM runtime), D5 (module interface) | R1.1-R1.4 |
| P2 | D6 (token/chain), D9 (chain target) | R5.1, new |
| P3 | D2 (CRDT approach) | R2.1-R2.2 |
| P4 | D3 (mesh transport) | R3.1-R3.5 |
| P5 | D9 (chain target — TON distribution) | new |
| P6 | Core thesis validation | new |
| P7 | D4 (DID/identity), D9 (multi-chain) | R4.1-R4.3 |
| P8 | Full stack feasibility (Layer 2) | new |
| P9 | Full stack feasibility (Layer 1) | new |
| P10 | Marketplace design (Phase 5) | R5.2 |
| P11 | D4 (DID method) | R4.1, R4.4 |
