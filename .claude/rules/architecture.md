# Architecture Rules

- All engine/module code depends ONLY on `rashk-core` traits. Never import adapter crates from engines.
- Port traits must work across all adapter targets: local, WASM, mesh, cloud, on-chain, browser.
- If a method signature assumes local execution (filesystem paths, OS primitives, tokio-specific types), rethink it.
- Every port method's types must be serializable (serde) for mesh/WASM boundary crossing.
- The composition root (`rashk-app`) is the only place that knows about concrete adapters.
- Prefer fewer, more general ports over many specialized ones. Specialized concerns become WASM modules.
