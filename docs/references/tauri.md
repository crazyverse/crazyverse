# Tauri — Reference Analysis

> Rust + Webview desktop app framework. Tiny binaries, native performance, web UI.

## Architecture

```
crates/
  tauri/                Main framework crate (app lifecycle, IPC, plugins, commands)
  tauri-runtime/        Abstract runtime trait (webview agnostic)
  tauri-runtime-wry/    WRY webview implementation
  tauri-build/          Build-time macros, resource embedding
  tauri-codegen/        Compile-time asset embedding, config parsing
  tauri-macros/         Proc macros (#[command], #[tray_icon], etc.)
  tauri-utils/          Shared utilities, config parsing, CSP
  tauri-bundler/        OS-specific binary bundling (DMG, MSI, AppImage, DEB)
  tauri-plugin/         Plugin trait and utilities
  tauri-cli/            CLI tool (init, dev, build)
packages/
  api/                  TypeScript API (@tauri-apps/api) — JS ↔ Rust bridge
```

### How It Works

```
1. Rust binary starts → creates webview window (via WRY/TAO)
2. Frontend (HTML/JS/CSS) loaded into webview
3. JS calls Rust via IPC:
   JS: invoke('my_command', { arg: 'value' })
   → WebView postMessage
   → Rust deserializes, calls #[command] fn
   → Result serialized back to JS
4. Rust can emit events to JS:
   app.emit('event_name', payload)
   → JS listener receives it
```

### Key Abstractions

| Concept | Tauri | RASHK Equivalent |
|---------|-------|------------------|
| App | `tauri::App` (lifecycle, state, router) | `rusvel-app` composition root |
| Command | `#[tauri::command]` (Rust fn callable from JS) | API route handlers |
| Plugin | `tauri::Plugin` (extend app with capabilities) | `DepartmentApp` |
| State | `tauri::State<T>` (shared state via DI) | `AppState` in rusvel-app |
| IPC | JSON-RPC over webview bridge | Axum HTTP (localhost) |
| Bundler | `tauri-bundler` (native installers) | `cargo build --release` |
| Capabilities | `capability` files (permission declarations) | `CapabilityPort` |
| Assets | `tauri-codegen` embeds at compile time | `rust-embed` (already used) |

## What to Extract for RASHK

### 1. Plugin System → Department/Module pattern
Tauri plugins have: `initialize()`, `on_event()`, `extend_api()`, permissions.
- Study: `crates/tauri-plugin/src/`
- Study: `crates/tauri/src/plugin/` (plugin manager)
- RASHK: DepartmentApp already does this — refine with Tauri's permission model

### 2. Capability Permissions → `rashk-core::CapabilityPort`
Tauri 2.0 has fine-grained permissions: each plugin declares capabilities, app grants them.
- Study: `crates/tauri-utils/src/acl/` (Access Control List)
- RASHK: `CapabilityPort::grant()` + `CapabilityGrant` already defined

### 3. IPC Pattern → Frontend ↔ Backend communication
Tauri's IPC is typed, serialized, and bidirectional (invoke + events).
- Study: `crates/tauri/src/ipc/` (Rust side)
- Study: `packages/api/src/` (TypeScript side)
- RASHK: currently uses HTTP (Axum) — could add WebSocket for lower latency

### 4. Asset Embedding → Binary distribution
`tauri-codegen` embeds frontend assets at compile time with compression.
- Study: `crates/tauri-codegen/src/`
- RASHK: already uses `rust-embed` for the same purpose

### 5. Native Bundling → Distribution
Cross-platform bundling: DMG (macOS), MSI/NSIS (Windows), AppImage/DEB (Linux).
- Study: `crates/tauri-bundler/src/`
- RASHK: useful when distributing the binary to end users

### 6. Auto-updater → Self-updating binary
Tauri has built-in update checking and applying.
- Study: Tauri updater plugin
- RASHK: important for "one binary that updates itself"

## Key Files to Study

```
crates/tauri/src/app.rs              # App lifecycle, setup, run loop
crates/tauri/src/ipc/                # IPC (invoke, events, channels)
crates/tauri/src/plugin/             # Plugin system
crates/tauri-utils/src/acl/          # Capability/permission model
crates/tauri-codegen/src/            # Asset embedding
crates/tauri-bundler/src/            # Cross-platform bundling
crates/tauri-runtime/src/lib.rs      # Abstract runtime trait
packages/api/src/                     # TypeScript bridge API
ARCHITECTURE.md                       # Official architecture doc
```

## Porting Priority: P0

Tauri's patterns directly inform RASHK's delivery model:
- Plugin system → DepartmentApp/ModuleRuntime
- Capabilities → CapabilityPort grants
- Asset embedding → already done with rust-embed
- IPC → consider WebSocket upgrade from HTTP
