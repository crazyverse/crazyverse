---
name: adr-create
description: Create a new Architecture Decision Record for rashk. Use when making architectural decisions about ports, WASM runtime, CRDT, mesh, identity, or any layer of the system.
allowed-tools: Read, Write, Glob, Bash, Grep
---

Create a new ADR in `docs/adrs/` following this process:

1. Find the next ADR number by listing existing files in `docs/adrs/`
2. Read `docs/MASTER_PLAN.md` to understand the current architecture context
3. Create a new file: `docs/adrs/NNNN-<slug>.md`
4. Use this template:

```markdown
# ADR-NNNN: $ARGUMENTS

**Status:** Proposed
**Date:** ${DATE}
**Phase:** (which phase from MASTER_PLAN.md this relates to)
**Research:** (link to docs/research/ notes if any)

## Context

What forces are at play? Reference:
- RUSVEL's existing approach (in /Users/bm/rusvel if relevant)
- RASHK's port trait design (in rashk-core)
- The specific phase requirements from MASTER_PLAN.md
- Research findings from Perplexity/Gemini/Claude

## Options Considered

| Option | Pros | Cons |
|--------|------|------|
| A | ... | ... |
| B | ... | ... |

## Decision

What we chose and why.

## Consequences

- What becomes easier
- What becomes harder
- What we defer
- Impact on port traits

## References

- Links to research, crate docs, prior art
```

5. Fill in all sections based on the user's description and any research notes in `docs/research/`
6. Keep it concise — two pages maximum
7. Update the Decisions Log table in `docs/MASTER_PLAN.md` with the new ADR reference
