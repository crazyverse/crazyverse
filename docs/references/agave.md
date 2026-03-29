# Agave (Solana Validator) — Reference Analysis

> Solana validator client by Anza. Rust monorepo, ~600+ crates.

## Architecture

```
sdk/                 Solana SDK — types, keypair, transaction building
  program/           #[program] macro, entrypoint, accounts
runtime/             Transaction execution runtime
  src/bank.rs        Bank — applies transactions to state
  src/accounts_db.rs AccountsDB — state storage
core/                Validator core (TPU, TVU, gossip)
programs/            Built-in programs (system, token, vote, stake)
cli/                 Solana CLI tool
client/              RPC client library
accounts-db/         Persistent account storage (append-only)
banking-stage-*/     Transaction ingestion pipeline
validator/           Validator binary entry point
gossip/              Gossip protocol for peer discovery
streamer/            UDP packet streaming
ledger/              Block/transaction persistence
```

### Key Concepts

| Concept | Solana/Agave | RASHK Equivalent |
|---------|-------------|------------------|
| Account | 32-byte address + data + owner + lamports | `Record` in rashk-core |
| Program | On-chain executable (BPF/SBF bytecode) | WASM module |
| Transaction | Signed instruction set | Agent action + approval |
| Keypair | Ed25519 signing key | `IdentityPort` keypair |
| RPC | JSON-RPC API for querying state | `PaymentPort` adapter calls |
| SPL Token | Fungible/non-fungible token standard | Payment primitive |

### Transaction Processing Pipeline

```
Client signs transaction
  → TPU receives via UDP/QUIC
  → Banking Stage:
    → Signature verification (GPU-accelerated)
    → Account locking (parallel execution)
    → Program execution (BPF VM)
    → State commitment
  → Block packing → Proof of History
  → Gossip propagation to other validators
```

## What to Extract for RASHK

### 1. Solana SDK for Payment Integration → `PaymentPort` adapter
The SDK is the client library for building and sending transactions.
- Study: `sdk/` — `Keypair`, `Transaction`, `Instruction`
- RASHK: `PaymentPort::send_payment()` wraps SDK calls

### 2. Account Model → Understand Solana state
Every account has: `lamports` (balance), `data` (arbitrary bytes), `owner` (program).
- Study: `sdk/account/src/` — Account struct
- RASHK: map `PaymentBalance` to Solana account queries

### 3. RPC Client → `PaymentPort` adapter implementation
JSON-RPC for `getBalance`, `sendTransaction`, `getSignatureStatuses`.
- Study: `client/src/rpc_client.rs`
- RASHK: use `solana-client` crate as dependency in PaymentPort adapter

### 4. Program Structure → Future RASHK on-chain programs
If RASHK ever deploys on-chain programs (escrow, reputation), this is the pattern.
- Study: `programs/system/src/` — simplest built-in program
- Study: `sdk/program/src/` — program SDK (entrypoint, accounts, sysvar)

### 5. Keypair Management → `rashk-identity`
Solana uses Ed25519 (same as RASHK). Keypair file format, derivation paths.
- Study: `sdk/signer/src/keypair.rs`
- RASHK: `rashk-identity` already uses `ed25519-dalek`

## Key Crates to Study

```
sdk/                              # Client SDK (types, keypair, transaction)
client/src/rpc_client.rs          # RPC client for querying Solana
programs/system/src/              # Simplest on-chain program
runtime/src/bank.rs               # How transactions get executed
accounts-db/                       # State storage patterns
```

## Concrete Integration Path

```rust
// In a new crate: rashk-solana/
// Implements PaymentPort for Solana

use solana_client::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, transaction::Transaction};
use rashk_core::ports::{PaymentPort, PaymentBalance, PaymentStatus};

struct SolanaPayment {
    rpc: RpcClient,
    keypair: Keypair,
}

#[async_trait]
impl PaymentPort for SolanaPayment {
    async fn balance(&self) -> Result<PaymentBalance> { /* rpc.get_balance() */ }
    async fn send_payment(&self, to, amount, memo) -> Result<String> { /* build + send tx */ }
    async fn verify_payment(&self, tx_id) -> Result<PaymentStatus> { /* rpc.get_signature_status() */ }
}
```

## Porting Priority: P3

Focus on: RPC client integration, keypair compatibility, transaction building.
Phase 5 work. Don't build on-chain programs until the runtime loop is proven.
