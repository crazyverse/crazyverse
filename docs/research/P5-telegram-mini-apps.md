# P5: Telegram Mini Apps as Distribution

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** D9 — Chain target / distribution channel

## Key Findings

- **Market size:** 242 Mini Apps using Monetag had 25.3M cumulative MAU (Jan 2026)
- **Framework support:** Framework-agnostic — React, Vue, Svelte, SvelteKit all work. SvelteKit has `@telegram-apps/sdk-svelte` package
- **TON Connect:** Mandatory for blockchain Mini Apps. Must not modify SDK, must support multiple wallets
- **PWA compatibility:** YES — same codebase with Telegram adapter layer + browser fallback
- **Revenue rules:** Digital goods MUST use Telegram Stars (~30-35% platform cost). Physical goods can use third-party. TON-native transactions (fees, tips) also viable
- **Approval:** No Apple-style review queue, but BotFather + TOS + blockchain rules enforced
- **Conversion rates:** No public benchmarks, but low-friction top-of-funnel (no install needed)
- **SvelteKit:** Supported and viable, but React has more documented examples

## RASHK Implications

- TON/Telegram confirmed as viable distribution channel
- SvelteKit works for Mini Apps — aligns with RASHK's frontend stack preference
- PWA-outside-Telegram means the same app serves both Telegram users and direct web users
- Stars cost (~30%) means some revenue models need off-platform monetization paths
- TON-native transactions (micropayments, tips, module purchases) bypass Stars for non-digital-goods
- Low-friction distribution (no app store install) is a strong acquisition advantage
