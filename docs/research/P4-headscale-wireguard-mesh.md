# P4: Headscale + WireGuard Mesh in Practice

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** D3 — Mesh transport

## Key Findings

- **Deployments:** 40-100 nodes on single VPS common, clustering via Postgres for larger scale
- **NAT traversal:** DERP "barely used" for 40+ clients, most connections direct WireGuard
- **Self-hosted DERP:** "Weekend project" level effort (TLS, DNS, JSON config)
- **Peer Relay:** NOT yet shipped in Headscale (tracking issue open). Use DERP or "bounce" VPS nodes instead
- **boringtun:** Cloudflare moved future to GotaTun fork, now developed by Mullvad (strong production signal)
- **defguard_wireguard_rs:** High-level Rust API for WireGuard interfaces, targets embedding in Rust apps

## Alternatives

| System | Model | Notes |
|--------|-------|-------|
| Headscale | Open-source Tailscale coordination | Best for MVP, proven model |
| NetBird | WireGuard + ZTNA | Self-hostable, higher-level |
| Nebula | Custom UDP, cert-based | Very scalable, worth watching for large fleets |
| ZeroTier | Proprietary-ish | Increasingly restrictive licensing |

## RASHK Implications

- Headscale confirmed for Phase 3 MVP — well-understood, adequate scale
- defguard_wireguard_rs or GotaTun for Phase 3+ pure-Rust WireGuard embedding
- Direct WireGuard connections dominate over relay — good for latency-sensitive CRDT sync
- Nebula worth tracking as alternative for large-fleet scenarios (100+ nodes)
- Self-hosted DERP is trivial, so no vendor dependency for relay infrastructure
