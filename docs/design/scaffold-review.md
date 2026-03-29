# Scaffold Review — 2026-03-27

Current state: 6 crates, ~1,400 lines, 7 tests.

## What's Solid

- **Port trait design** — 16 async traits, clean, minimal, composable, all `Send + Sync`. Hardest part to get right, and it's done well.
- **Ed25519 crypto** — Correct usage of `ed25519-dalek` with OS entropy. Sign/verify round-trip tested.
- **WIT interface** — Guest/host contract (`handle`, `list-tools`, `store-get/put`, `emit-event`) is the right abstraction for sandboxed modules.
- **CRDT-readiness** — Every `Record` has HLC + `origin_node` from day one. No painful migration later.
- **Composition root** — All adapter wiring in `rashk-app/main.rs`. Engine code depends only on port traits.

## What's Stubbed

| Component | Status |
|---|---|
| `WasmHost::call()` | Returns stub JSON — modules can't execute |
| `QuicMesh::send()` | Returns error — no mesh communication |
| `QuicMesh::connect()` | Logs and does nothing |
| Identity persistence | New keypair generated every run |
| WASI context | Not wired — module sandbox incomplete |
| BlobStore ops | Table created, not implemented in SqliteStore |
| search_index | FTS5 table created, never populated |
| DID resolution | Returns `None` (Phase 5) |

## Bugs & Risks

- **SQL injection** in `rashk-store` query builder — WHERE clauses built with string concatenation instead of parameterized queries.
- **Silent data loss** — `.unwrap_or_default()` on JSON parse in store silently drops malformed records.
- **No key persistence** — Identity regenerates on every `rashk` invocation, breaking any trust/signing workflow.

## Code Debt

- **Unused dependencies** bloating compile times: `solana-sdk`, `axum`, `tower`, `tower-http`, `reqwest`, `quinn`, `rustls`, `cr-sqlite-bundle`, `iroh-net`, `wasmtime-wasi`, `wit-bindgen`
- **Error handling** — `RashkError` enum lacks context chaining. Errors lose their source.
- **Tests are thin** — 7 total, mostly "does it construct without panicking." Only identity sign/verify is meaningful.

## Priorities

1. **Persist identity to disk** — Save/load keypair from `~/.rashk/identity.key`. Without this, nothing downstream works.
2. **Implement `WasmHost::call()`** — Wire WASI context, bind host functions, execute guest `handle()`. This is the core value proposition.
3. **Fix SQL injection** — Use parameterized queries in `SqliteStore::query()`.
4. **Strip unused deps** — Remove everything not yet imported. Add back when actually needed.
5. **Integration tests** — At minimum: Store put → query → get round-trip, and identity save → load → sign → verify.

## Bottom Line

The architecture is right. The port traits are production-grade abstractions. But until one vertical slice works end-to-end — install a WASM module, call it, have it read/write the store — this is a design document expressed in Rust syntax. Focus on depth over width.
