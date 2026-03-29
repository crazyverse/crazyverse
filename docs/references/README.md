# Reference Repos — Study & Porting Guide

## Overview

8 open-source projects cloned into `ref-repos/` as architectural references for RASHK-RUS.
Each maps to specific RASHK layers and crates.

## Reference Map

```
RASHK Layer          Reference Repo       What We Learn
─────────────────    ─────────────────    ─────────────────────────────
Layer 1: Runtime     tauri                Single binary, Rust+webview IPC, plugin system
                     claude-code          Agent loop, tool system, MCP protocol

Layer 2: WASM        tauri                Plugin/capability permission model
                     supabase             Edge functions (Deno/WASM workers)

Layer 3: Mesh        tailscale            WireGuard userspace, DERP relay, NAT traversal
                     ton                  ADNL protocol, DHT, overlay networks

Layer 4: Identity    ton                  Wallet model, smart contracts, crypto identity
                     agave                Account model, program architecture

Layer 5: AI          claude-code          Agent runtime, tool-use loop, streaming
                     everything-claude-code  Claude Code best practices, skills, hooks

Cross-cutting:
  Backend API        supabase             Realtime, RLS, storage, auth
  Workflows          n8n                  DAG execution, node types, credentials
  Payments           agave + ton          On-chain transactions, escrow, micropayments
```

## Per-Repo Docs

- [supabase.md](supabase.md) — Backend-as-a-service patterns
- [n8n.md](n8n.md) — Workflow automation engine
- [tailscale.md](tailscale.md) — Mesh networking
- [ton.md](ton.md) — TON blockchain & ADNL networking
- [agave.md](agave.md) — Solana validator & payment integration
- [claude-code.md](claude-code.md) — AI agent runtime & MCP
- [tauri.md](tauri.md) — Single binary desktop app framework

## Porting Priority

| Priority | Repo | RASHK Target | Why |
|----------|------|-------------|-----|
| P0 | claude-code | rusvel-agent | Agent loop is the kernel — everything flows through it |
| P0 | tauri | rusvel-app | Single binary + webview is our delivery model |
| P1 | n8n | flow-engine | Workflow execution is how agents compose work |
| P1 | supabase | rashk-store, rusvel-api | Realtime + RLS patterns needed for multi-user |
| P2 | tailscale | rashk-mesh | Mesh networking enables P2P (Phase 3) |
| P3 | agave | PaymentPort | Solana payments (Phase 5) |
| P3 | ton | rashk-identity, PaymentPort | ADNL + wallet model (Phase 4-5) |
