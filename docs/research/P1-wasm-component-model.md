# P1: WASM Component Model in Production

**Source:** Perplexity, 2026-03-29
**Status:** DONE
**Informs:** D1 (WASM runtime), D5 (module interface)

---

## Summary

Rust hosts using WIT and Component Model are in production but still "early adopter" tier, not boring-at-scale. Supabase Wrappers is the strongest production reference. Fermyon Spin and Cosmonic are the main platform adopters.

## Key Findings

### Rust hosts using `wasmtime::component::bindgen!`

- Rust is the most mature language for hosting WIT-based components
- `wasmtime::component::bindgen!` is the primary path for custom WIT worlds
- Typical pattern: define WIT world → `bindgen!` generates typed traits → load components via `wasmtime::Component` + `Linker`
- WIT resources map to host-side Rust types via `bindgen!`'s `with` field (`Arc<T>`, IDs, custom wrappers)
- Public examples are mostly frameworks/infra projects, not "BigCo X runs this for 10k TPS" writeups

### Supabase Wrappers (production reference)

- Rust FDW framework for Postgres with first-class Wasm FDW support
- Embeds Wasm runtime inside the Postgres process
- WIT package (`supabase-wrappers-wit`) defines host capabilities (HTTP, logging, etc.)
- Guest FDWs implement their own WIT world depending on the Supabase package
- **Production FDWs:** OpenWeather, NTP, Energy Charts, Gravatar — running on Supabase platform
- Reports "near-native performance" for production workloads

**Lessons from Supabase Wrappers:**
- Keep FDWs stateless; rely on HTTP and caching
- Design WIT interfaces for batched/streaming responses, not per-row calls
- Version the WIT package and keep it stable — FDWs break across upgrades

### Fermyon Spin

- Spin 2.x supports WASI 0.2.0 components since Spin 2.2
- Apps composed of multiple components wired by WIT interfaces
- Positioned as production-ready serverless infra
- Rust is one of the main implementation languages

### Cosmonic (wasmCloud)

- Emphasizes WIT for contracts between "actors" and "capabilities"
- Custom interfaces for key-value, messaging, logging
- Positions as production multi-tenant PaaS

### Other Rust hosts

- Hyperlight (Microsoft): Wasm guest on Wasmtime without guest OS, 1-2ms cold starts
- Several infra projects use WIT + `bindgen!` but don't brand it as "component model" publicly

## Performance

| Metric | Value | Source |
|--------|-------|--------|
| Host-guest call | ~100 microseconds per call (optimized build) | Bevy discussion |
| Cold start | 1-2ms (vs 125ms+ VM, 100ms+ container) | Hyperlight, various |
| Compute vs native | 0.5-0.9x | Various benchmarks |
| Memory | Lower than containers (multi-tenancy) | Multiple sources |

**Critical:** Host-guest boundary copies dominate for large payloads. Design WIT interfaces to batch work, use handles instead of large data, stream chunks.

## Gotchas for Custom WIT

1. **Versioning:** Changing a WIT world is a breaking change. Need versioned packages (`my-abi@v1`, `my-abi@v2`).
2. **Resources:** WIT resources must map to Send+Sync Rust types for async/multi-threaded hosts. Design explicit close/dispose operations.
3. **Large payloads:** Strings and lists are copied via canonical ABI. Pass handles/IDs instead, fetch via separate I/O interface.
4. **Error handling:** Large error enums with complex records cost on every failing call. Keep error surfaces narrow on hot paths.
5. **WASI interplay:** Mixing custom WIT with WASI interfaces creates complex dependency graphs. Careful with package imports.
6. **Tooling:** Only Rust + Wasmtime has first-class support. Other languages catching up.
7. **Error messages:** WIT parser and `bindgen!` errors can be terse. Debugging malformed packages takes time.

## Recommendation Pattern for RASHK

- Define narrow "service" WIT worlds with coarse-grained operations + streaming
- Use WIT resources for long-lived contexts (sessions, cursors) with explicit close
- Keep payload types simple: records of scalars, small lists, short strings
- Introduce versioned worlds for ABI evolution
- Study Supabase Wrappers WIT structure before designing RASHK's

## RASHK Decision Impact

**D1 confirmed: wasmtime directly.** The ecosystem is built around wasmtime + `bindgen!`. Extism and wasmer are not where the component model investment is happening.

**D5 confirmed: WIT Component Model.** WASI 0.2 is the right target. Custom WIT worlds for RASHK port traits are proven by Supabase Wrappers and Spin.

---

*Sources: platform.uno, github.com/bytecodealliance/wit-bindgen, docs.wasmtime.dev, supabase.com, fermyon.com, cosmonic.com, eunomia.dev, news.ycombinator.com*
