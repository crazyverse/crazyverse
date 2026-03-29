# Porting Roadmap — Reference Repos to RASHK

## Phase 1: Runtime + Agent Core (NOW)

### From claude-code + everything-claude-code → `rusvel-agent`
**Goal:** Best-in-class agent loop

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Port 30 agent definitions to persona format | `everything-claude-code/agents/*.md` | `rusvel-agent/personas/` | S |
| Add loop-operator pattern (recurring agents) | `agents/loop-operator.md` | `rusvel-cron` + `rusvel-agent` | M |
| Refine skill system with community patterns | `everything-claude-code/skills/` | Department skills | S |
| Study Claude Code plugin architecture | `claude-code/plugins/` | WASM module manifest pattern | S |

### From tauri → `rusvel-app`
**Goal:** Production-quality single binary

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Study capability/permission model | `tauri-utils/src/acl/` | `rashk-core::CapabilityPort` | M |
| Study plugin lifecycle (init, event, api) | `tauri/src/plugin/` | `DepartmentApp` refinement | M |
| Consider WebSocket IPC (faster than HTTP) | `tauri/src/ipc/` | `rusvel-api` WebSocket upgrade | L |
| Study auto-updater pattern | Tauri updater plugin | Self-updating binary | M |

---

## Phase 2: Workflow Engine Enhancement

### From n8n → `flow-engine`
**Goal:** Production workflow execution

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Port node type interface | `workflow/src/Interfaces.ts` → `INodeType` | `flow-engine` node trait | M |
| Add credential encryption | `core/src/CredentialTypes.ts` | `rusvel-auth` + `IdentityPort` | M |
| Improve DAG execution with n8n patterns | `core/src/WorkflowExecute.ts` | `flow-engine/src/executor.rs` | L |
| Study webhook trigger patterns | `cli/src/webhooks/` | `rusvel-webhook` refinement | S |
| Port visual builder concepts | `frontend/src/components/canvas/` | `WorkflowBuilder.svelte` | L |

---

## Phase 3: Backend-as-a-Service Layer

### From supabase → `rashk-store`, `rusvel-api`
**Goal:** Realtime, RLS-like security, schema introspection

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Study RLS → design capability-based record access | PostgREST RLS model | `CapabilityPort` + `Store` | L |
| Add realtime subscriptions (CDC pattern) | `supabase/realtime` | `EventBus` + SSE/WebSocket | L |
| Enhance schema introspection | `packages/pg-meta/` | `rusvel-schema` | M |
| Study storage API patterns | `supabase/storage-api` | `BlobStore` adapter | M |

---

## Phase 4: Mesh Networking

### From tailscale → `rashk-mesh`
**Goal:** P2P encrypted networking with NAT traversal

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Implement NAT traversal (STUN + fallback) | `magicsock/magicsock.go` | `rashk-mesh/src/quic.rs` | XL |
| Build DERP-like relay server | `derp/derp.go` | New: `rashk-relay` crate | L |
| Implement peer discovery (mDNS + coordination) | `netcheck/`, `control/` | `MeshPort::discover()` | L |
| Study NetMap for peer state sync | `types/netmap/` | `MeshPort::peers()` enrichment | M |

### From TON → `rashk-mesh` (ADNL patterns)
**Goal:** Abstract addressing, DHT discovery

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Adopt hash-of-pubkey as NodeId | `adnl/adnl-node-id.cpp` | `rashk-core::types::NodeId` | S |
| Study DHT for WAN peer discovery | `dht/dht-query.cpp` | `MeshPort` DHT adapter | L |
| Study overlay pub/sub pattern | `overlay/overlay.cpp` | `EventBus` cross-node | M |

---

## Phase 5: Identity & Payments

### From TON → `rashk-identity`
**Goal:** Self-sovereign identity

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Study wallet-as-smart-contract pattern | `crypto/smartcont/` | Identity model design | S |
| Evaluate DID method (did:key, did:web, did:ton) | TON docs + research | `IdentityPort::resolve_did()` | M |

### From agave → `PaymentPort` adapter
**Goal:** Solana payment integration

| Task | Source | Target | Effort |
|------|--------|--------|--------|
| Create `rashk-solana` crate | `sdk/`, `client/` | New crate implementing `PaymentPort` | M |
| Implement balance query | `client/src/rpc_client.rs` | `PaymentPort::balance()` | S |
| Implement send transaction | `sdk/transaction/` | `PaymentPort::send_payment()` | M |
| Implement verification | `client/src/rpc_client.rs` | `PaymentPort::verify_payment()` | S |
| Study program model for escrow | `programs/system/` | Future escrow contract | L |

---

## Effort Legend

- **S** = Small (< 1 day)
- **M** = Medium (1-3 days)
- **L** = Large (3-7 days)
- **XL** = Extra Large (1-2 weeks)

## Dependency Graph

```
Phase 1 (Agent + Binary)
  ├── No dependencies, start now
  └── Enables: everything else

Phase 2 (Workflows)
  ├── Depends on: Phase 1 (agent loop)
  └── Enables: automated pipelines

Phase 3 (Backend)
  ├── Depends on: Phase 1 (store + API)
  └── Enables: multi-user, realtime

Phase 4 (Mesh)
  ├── Depends on: Phase 1 (identity)
  └── Enables: P2P, distributed

Phase 5 (Payments)
  ├── Depends on: Phase 4 (mesh for discovery)
  └── Enables: marketplace economy
```

## What NOT to Port

- **n8n's 400+ node implementations** — RASHK uses WASM modules, not compiled-in nodes
- **Supabase's Docker composition** — RASHK is a single binary, not microservices
- **Tailscale's WireGuard kernel module** — RASHK uses QUIC userspace, not WireGuard
- **TON's TVM/FunC** — RASHK uses WASM, not a custom VM
- **Agave's validator/consensus** — RASHK is a client, not a validator
- **Tauri's WRY/TAO webview** — RASHK uses embedded SvelteKit, not native webview
