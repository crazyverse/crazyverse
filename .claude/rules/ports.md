# Port Trait Design Rules

When designing or modifying port traits in `rashk-core`:

- Every trait method must be `async` (remote adapters need it).
- Return types must be `Result<T>` with errors that cover: not found, permission denied, timeout, network error, serialization error.
- All parameter and return types must derive `Serialize + Deserialize` (for WASM and mesh boundary).
- No `std::path::Path` in port signatures — use string keys or URIs instead.
- No `tokio::sync::broadcast::Receiver` or other runtime-specific types — use trait-defined stream types.
- Include pagination (`offset`/`limit` or cursor) on any method that returns `Vec<T>`.
- Use `bytes::Bytes` for binary data, not `Vec<u8>` (zero-copy across boundaries).
- Keep traits small — 3-7 methods max. Split large traits into composable pieces.
- Use `/port-audit` skill to verify universality before finalizing.
