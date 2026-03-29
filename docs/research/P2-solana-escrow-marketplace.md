# P2: Solana Escrow & Service Marketplace Programs

**Source:** Perplexity, 2026-03-29
**Status:** DONE
**Informs:** D6 (token/chain), D9 (chain target)

---

## Summary

Solana has strong escrow primitives in Anchor/Rust but polished freelance-marketplace end-to-end protocols are still sparse. Milestone-gated escrow exists but isn't standardized. On-chain reputation is thin. PDA-based entity registration is clean and well-documented. Agent-initiated transactions work via key/MPC/relayer patterns.

## Key Findings

### Existing Programs

**Escrow (well-established):**
- `ironaddicteddog/anchor-escrow` — standard vault/PDA/token-account pattern
- `solanakite/anchor-escrow-2026` — newer starter
- Turbin3's 2025 escrow example
- All are bilateral escrow/swap, not full milestone-based

**Service/Freelance contracts (emerging):**
- `snndmaa/solana_escrow_contract` — job escrow with employer/worker approval
- `manustays/trusted-properties-marketplace-solana-anchor` — prepaid rental marketplace
- TasteMaker stack (2026) — milestone-gated escrow + voting-based release, devnet

**Milestone-gated:** Pattern is becoming real in production-style builds but NOT yet a canonical standardized Anchor repo.

### Reputation Systems

- Public Solana repos for on-chain reviews/SBT reputation are **much thinner** than escrow
- No established Anchor standard for decentralized reviews
- Teams often store reputation off-chain or issue program-scoped attestations/NFTs
- Review data is storage-heavy and moderation-sensitive

**Practical patterns in 2026:**
- Review PDA keyed by `(platform, job, reviewer)` — score + hash/URI to off-chain content
- Non-transferable badge/SBT mint after successful completion
- Aggregated counters on provider profile PDA, detailed evidence off-chain

### PDA Registration Model

PDAs are THE natural way to register marketplace entities on Solana:

```
Provider profile PDA:  seeds = ["provider", provider_wallet]
Service listing PDA:   seeds = ["service", provider_wallet, service_slug]
Job/engagement PDA:    seeds = ["job", buyer_wallet, provider_wallet, nonce]
Milestone PDA:         seeds = ["milestone", job_pda, milestone_index]
Vault authority PDA:   seeds = ["vault", job_pda]
```

Advantages: deterministic addresses, synchronous registration in one tx, clean namespacing by seeds.

### Gas Costs

| Step | Cost |
|------|------|
| Base fee per signature | 5,000 lamports |
| Create escrow (1 sig + PDA rent) | Base fee + rent (~0.002 SOL for account) |
| Fund escrow (1 sig + token transfer) | Base fee (+ ATA creation if needed) |
| Release escrow (1 sig + CPI transfer) | Base fee |
| **Three-step total (base fees only)** | **~15,000 lamports** |

ATA (Associated Token Account) creation dominates cost when needed. Priority fees are optional.

### Agent-Initiated Transactions

Three production patterns for agents on Solana:
1. **Agent holds software key** — simplest, weakest operationally
2. **Agent via MPC/embedded wallet** — stronger, closer to production (Helius docs)
3. **Agent triggers backend relayer** — relayer/delegated wallet is actual signer

No signatureless native agent primitive exists — agents must control or request a signer.

### Solana vs TON for Escrow

| Aspect | Solana | TON |
|--------|--------|-----|
| State update | Synchronous, one tx mutates all accounts | Async messages, multi-step |
| Entity registration | PDA seeds, deterministic | Contracts or message-addressed state |
| Milestone release | Clean: milestone PDAs + release instruction | Sequence of query/response messages |
| Failure handling | Transaction-level atomicity | Must handle bounced messages, partial state |
| Agent operation | Signs or controls signer | Dispatches messages (async-native) |

**Verdict:** Solana is easier for single-program escrow with milestones. TON's async model demands more care but is natural for message-driven orchestration.

## RASHK Decision Impact

**D6 direction: Solana for contracts.** PDA-based entity/service/job/milestone registration is clean and maps directly to RASHK's Universe Model primitives.

**D9 direction: Solana primary for on-chain logic.** TON for distribution (Telegram). The escrow/marketplace contracts should be Anchor/Rust on Solana.

**Action item:** Design the Anchor account model for RASHK: `Entity`, `Service`, `Contract`, `Milestone`, `Review`, `VaultAuthority` PDAs.

---

*Sources: github.com (anchor-escrow, solana_escrow_contract, trusted-properties-marketplace), solana.com/docs, helius.dev, alchemy.com, docs.ton.org, reddit.com/r/solana*
