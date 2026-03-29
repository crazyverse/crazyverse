//! rashk-wasm: WASM module host.
//!
//! Uses wasmtime to load, sandbox, and execute capability modules.
//! Each module gets explicit capability grants via WASI.

mod host;

pub use host::WasmHost;
