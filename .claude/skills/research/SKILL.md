---
name: research
description: Research a technical question from the MASTER_PLAN. Formats prompts for external AI tools (Perplexity/Gemini), logs findings to docs/research/.
allowed-tools: Read, Write, Glob, Grep, WebSearch, WebFetch
---

Research a technical question for rashk. Process:

1. Read `docs/MASTER_PLAN.md` section 4 (Research Questions) to find the relevant question
2. If `$ARGUMENTS` specifies a question ID (e.g., "R1.1"), look it up directly
3. If `$ARGUMENTS` is a topic (e.g., "wasmtime vs extism"), find matching questions

## For each question:

### Step 1: Search
- Use WebSearch/WebFetch to find current information
- Focus on: Rust ecosystem, crate maturity, production usage, benchmarks, known limitations
- Check crates.io download counts, GitHub stars, last commit date

### Step 2: Log findings
Create or update `docs/research/<question-id>.md`:

```markdown
# <Question ID>: <Question Title>

**Researched:** <date>
**Sources:** <list URLs>
**Confidence:** High / Medium / Low

## Findings

<concise summary of what we learned>

## Key Data Points

- Crate versions, maturity, download counts
- Benchmarks or performance data
- Known limitations or gotchas
- Production users / case studies

## Recommendation

<what this means for rashk's architecture>

## Prompts for External Tools

### For Perplexity
<formatted question optimized for Perplexity's web search>

### For Gemini
<formatted question optimized for Gemini's analysis>
```

### Step 3: Update MASTER_PLAN.md
If findings inform a decision, note it in the Decisions Log.

## Output
Summarize findings and provide the formatted prompts for the user to paste into Perplexity/Gemini.
