---
name: port-audit
description: Audit a port trait to ensure it works across all adapter targets (local, WASM, mesh, cloud, on-chain, browser). Use when designing or reviewing port traits.
allowed-tools: Read, Grep, Glob, Write
---

Audit a port trait for universality. Process:

1. Read the trait definition from `crates/rashk-core/src/ports.rs`
2. If `$ARGUMENTS` names a specific trait, audit that one. Otherwise audit all.
3. Read `docs/MASTER_PLAN.md` section 3 (Port Trait Audit) for context

## For each trait, check:

### Adapter Compatibility Matrix

Does every method work when the adapter is:

| Target | Can implement? | Issues |
|--------|---------------|--------|
| Local (SQLite, fs) | ? | ? |
| WASM sandbox (wasmtime guest) | ? | ? |
| Mesh proxy (remote node) | ? | ? |
| Cloud service (managed API) | ? | ? |
| On-chain (Solana program) | ? | ? |
| Browser (wasm-bindgen) | ? | ? |

### Red flags to check:
- **Path/filesystem references** — won't work in WASM or browser
- **Tokio-specific types** (broadcast::Receiver) — may not work in WASM guest
- **Large payloads in signatures** — bad for mesh/remote proxying
- **Missing pagination** — won't scale for distributed stores
- **Sync methods** — should everything be async for remote adapters?
- **Missing error context** — remote adapters need richer errors (network, timeout, permission)
- **Concrete types instead of traits** — limits adapter flexibility

### Compare with RUSVEL
If the trait evolved from a RUSVEL port, read the original in `/Users/bm/rusvel/crates/rusvel-core/src/ports.rs` and note:
- What was simplified and why
- What was lost that might be needed
- What was added for the new layers

## Output

Write audit results to `docs/research/port-audit-<trait-name>.md` and summarize issues found.
