---
name: phase-check
description: Check current phase status from MASTER_PLAN.md. Shows what's done, what's in progress, what's next.
allowed-tools: Read, Glob, Grep
---

Check rashk project phase status:

1. Read `docs/MASTER_PLAN.md`
2. Find the current phase (look for incomplete checkboxes)
3. Count completed vs total tasks per phase
4. Check `docs/adrs/` for written ADRs
5. Check `docs/research/` for completed research
6. Check `crates/` for implemented code

## Output format:

```
RASHK Phase Status
==================

Current Phase: Phase X — <name>
Progress: N/M tasks complete

Completed:
  [x] Task description

In Progress:
  [-] Task description (evidence: ...)

Not Started:
  [ ] Task description

Decisions:
  D1: <status> — ADR written? Research done?
  D2: ...

Blockers:
  - <anything blocking progress>

Suggested Next Action:
  <the single most impactful thing to do next>
```
