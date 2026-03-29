# P8: Chromium Fork Complexity

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** Full stack feasibility (Layer 2 — browser)

## Key Findings

- **Brave:** Hundreds of employees, dozens dedicated to browser/engine work
- **Vivaldi:** 8-week rebase cycle takes "just over two weeks". 4-week cycle would need "at least two full teams"
- **Realistic lower bound:** 50+ engineers for Brave/Vivaldi-class fork maintenance
- **Build time:** 3-4 DAYS on standard MacBook. Need beefy Linux build machines + remote caching
- **Release cadence:** Chromium ships 4-week stable + 8-week Extended Stable
- **Custom protocols:** `brave://` etc registered in C++ embedder layer, not web APIs
- **CEF:** Chromium Embedded Framework — stable embedding without maintaining fork, used by League of Legends
- **Tauri:** System WebView + Rust backend, dramatically easier for small teams, less control

## Honest Assessment

Chromium fork is NOT feasible for a 1-5 person team long-term.

## Feasible Alternatives

| Approach | Effort | Control |
|----------|--------|---------|
| Tauri (system WebView + Rust) | Low | Medium |
| CEF embedding | Medium | Medium-High |
| Electron | Medium | Medium |
| Full Chromium fork | Very High (50+ engineers) | Full |

## RASHK Implications

- RashkBrowser should be Tauri-based (Rust backend + system WebView), NOT a Chromium fork
- Deep browser features (custom protocols, native wallet) can be added incrementally via Tauri plugins
- Chromium fork only viable if/when team grows to 20+ engineers
- CEF is a middle ground if more browser control is needed than Tauri provides
- This decision removes a massive engineering risk from the roadmap
