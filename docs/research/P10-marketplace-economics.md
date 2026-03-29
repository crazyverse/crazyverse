# P10: Module Marketplace Economics

> Research prompt: What are the economics of software module/plugin marketplaces?

## Key Findings

### Existing Marketplace Models

**npm / Docker Hub** — Pure distribution, no money for authors. Authors monetize indirectly via consulting, SaaS, sponsorships. The "open-source tax" model: build for free, hope for adjacent revenue.

**Shopify App Store** — 0% commission on first $1M lifetime revenue, then 15%. Paid out $1B+/year to ~16K apps. Revenue distribution is very skewed (top apps earn massively, long tail earns little).

**WordPress** — Official plugin directory is free discovery only. Monetization happens via freemium model + external payment processors (Freemius, own site). CodeCanyon marketplace takes ~30-37.5% commission.

**Figma** — No built-in paid store. Authors monetize externally via Stripe or similar. Some successful plugin authors make $30K+/year.

**Apple App Store / Google Play** — 15-30% commission. Small Business Program at 15% for developers earning under threshold.

**Chrome Web Store** — ~5% transaction fee. Lower commission but smaller paid ecosystem.

**Crypto marketplaces** — 0-10% protocol fees. On-chain splits enable instant settlement with no reconciliation overhead. Transparent fee structures enforced by smart contracts.

### Key Insight

If you want developers to earn meaningful income, monetization must be a **first-class primitive** in the platform, not bolted on after the fact. npm proves that distribution alone does not create income. Shopify proves that integrated payments + discovery + commission structure can create a billion-dollar ecosystem for developers.

## RASHK Marketplace Design

### Commission Structure
- **0-2%** on first X volume (encourage adoption, lower barrier than Shopify's 0% which resets)
- **5-10%** above threshold
- Encoded in smart contracts — no surprise fee changes possible without governance

### On-Chain Payment Splits
- **90%** to module author
- **5%** to protocol treasury (funds development)
- **5%** to referrer (incentivizes curation and discovery)
- All splits are transparent and verifiable on-chain

### Payment Models
- **Stablecoin payments** (USDC on Solana) for predictability
- Multiple monetization models supported:
  - One-time purchase
  - Subscription (recurring on-chain payments)
  - Usage-based (metered via WASM runtime, settled periodically)
  - Freemium (free tier + paid capabilities)

### Governance
- DAO-controlled fee parameters to prevent surprise changes
- Module authors vote on marketplace policy changes
- Fee changes require supermajority + time-lock

### Discovery
- **Revenue-weighted satisfaction ranking** — not just install counts, but quality signal weighted by paying users
- **"Earners" leaderboard** — showcase modules that generate real income for authors
- **Third-party curation modules** — curators can build discovery experiences and earn referral share (the 5% referrer split)

## Decision Impact

Informs marketplace design for Phase 5. Key architectural requirement: payment splits and module licensing must be enforceable at the WASM runtime level (capability-based access tied to payment status).
