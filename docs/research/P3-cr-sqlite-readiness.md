# P3: cr-sqlite Production Readiness

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** D2 — CRDT approach

## Key Findings

- **Version/maturity:** Pre-1.0 (v0.16.3, Jan 2024), research-grade, not enterprise-ready
- **Maintainer:** Matt Wonlaw, GitHub Sponsors funded, ~13 contributors, 3.4k stars
- **CRDT types supported:** LWW registers, fractional indices, OR-set semantics. Counters and rich-text "still being implemented"
- **Schema migration:** Supported via `crsql_begin_alter` / `crsql_commit_alter` wrapper
- **Production deployments:** No published case studies at scale
- **WASM/browser:** Works via vlcn-io/js repo, can use OPFS for persistence
- **Rust integration:** Loads as SQLite extension via rusqlite — NOT a native Rust crate with idiomatic API

## Alternatives Comparison

| System | Strengths | Weaknesses |
|--------|-----------|------------|
| cr-sqlite | SQLite-native, works everywhere SQLite does | Pre-1.0, small team, limited CRDT types |
| ElectricSQL | More production-ready, batteries-included sync | Postgres-centric, conflicts with SQLite-everywhere goal |
| Automerge 2.0 | Better for nested documents | Weaker for relational queries |

## RASHK Implications

- cr-sqlite is capable but risky for production — small team, no enterprise adoption
- ElectricSQL is safer but Postgres-centric, which conflicts with RASHK's SQLite-everywhere goal
- May need a custom CRDT layer on top of SQLite rather than depending on cr-sqlite directly
- The rusqlite extension loading approach is workable but not ideal for a core dependency
- LWW + OR-set covers basic business data (invoices, tasks, contacts) but missing counters is a gap
