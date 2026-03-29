# Project Conventions

- Package manager: pnpm (never npm) for frontend
- Package manager: uv (never pip) for Python
- Package manager: cargo for Rust
- Rust: idiomatic, thiserror for lib errors, anyhow in binaries
- Conventional commits: feat:, fix:, refactor:, docs:, chore:, test:
- Atomic commits — one logical change per commit
- Concise code, no unnecessary comments, small functions
- No docstrings on obvious functions
- Testing: cargo test (Rust), vitest (TS/Svelte), pytest (Python)
