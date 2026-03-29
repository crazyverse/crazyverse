# Claude Code — Reference Analysis

> Anthropic's official CLI for Claude. AI agent with tool use, MCP, streaming.

## Official Claude Code (`ref-repos/claude-code/`)

### Structure

```
plugins/               Plugin templates for different use cases
  agent-sdk-dev/       Agent SDK development patterns
  code-review/         Code review plugin
  commit-commands/     Commit message automation
  feature-dev/         Feature development patterns
  frontend-design/     Frontend design assistance
  hookify/             Hook creation automation
  plugin-dev/          Plugin development guide
  pr-review-toolkit/   PR review tools
  security-guidance/   Security review patterns
examples/
  hooks/               Hook configuration examples
  settings/            Settings configuration examples
scripts/               Internal scripts
```

### Key Patterns for RASHK

1. **Plugin System**: Each plugin is a directory with `SKILL.md` defining behavior
2. **Hooks**: Shell commands triggered by events (tool calls, message submission)
3. **Settings**: JSON config for permissions, model selection, tool filtering
4. **Agent SDK**: TypeScript SDK for building custom agents

## Everything Claude Code (`ref-repos/everything-claude-code/`)

### Structure (community reference — much more detailed)

```
agents/                30+ specialized agent definitions
  architect.md         System architecture planning
  code-reviewer.md     Code review
  rust-reviewer.md     Rust-specific review
  security-reviewer.md Security audit
  planner.md           Task planning
  tdd-guide.md         Test-driven development
  loop-operator.md     Recurring task execution
skills/                Skill definitions (slash commands)
rules/                 Auto-loaded rules by file path
hooks/                 Event-driven shell automation
commands/              Custom CLI commands
contexts/              Context injection patterns
plugins/               Plugin system patterns
mcp-configs/           MCP server configurations
manifests/             Manifest definitions
docs/                  Comprehensive documentation
research/              Research on Claude Code internals
```

### Agent Pattern (most relevant for RASHK)

Each agent is a markdown file defining:
- **Role**: What the agent specializes in
- **Tools available**: Which tools the agent can use
- **Instructions**: How the agent should behave
- **Examples**: Sample interactions

This maps directly to RASHK's persona system in `rusvel-agent`.

## What to Extract for RASHK

### 1. Agent Definition Pattern → `rusvel-agent` personas
Claude Code defines agents as markdown files with structured sections.
- Study: `everything-claude-code/agents/*.md`
- RASHK: extend `PersonaConfig` to support this format

### 2. Skill System → Department skills
Skills are slash commands with `SKILL.md` defining behavior.
- Study: `everything-claude-code/skills/`
- RASHK: already has `resolve_skill()` with `{{input}}` interpolation

### 3. Hook System → Event-driven automation
Hooks fire shell commands on events (pre-tool-call, post-message, etc.).
- Study: `claude-code/examples/hooks/`, `everything-claude-code/hooks/`
- RASHK: already has hook dispatch via `tokio::spawn`

### 4. Rules System → Context-aware prompting
Rules auto-load based on file paths being edited.
- Study: `everything-claude-code/rules/`
- RASHK: already has `load_rules_for_engine()` pattern

### 5. MCP Integration → `rusvel-mcp` + `rusvel-mcp-client`
MCP (Model Context Protocol) for tool discovery across servers.
- Study: `everything-claude-code/mcp-configs/`
- RASHK: MCP server exists, MCP client exists

### 6. Plugin Architecture → WASM module pattern
Plugins package agents + skills + rules + hooks as reusable bundles.
- Study: `claude-code/plugins/`, `everything-claude-code/plugins/`
- RASHK: this maps to `DepartmentApp` manifests

### 7. Loop Operator Pattern → Recurring agent tasks
`loop-operator.md` defines an agent that runs tasks on intervals.
- Study: `everything-claude-code/agents/loop-operator.md`
- RASHK: `rusvel-cron` + job queue for recurring agent work

## Key Files to Study

```
# Official
claude-code/plugins/agent-sdk-dev/     # Agent SDK patterns
claude-code/examples/hooks/            # Hook configuration
claude-code/examples/settings/         # Settings patterns

# Community
everything-claude-code/agents/         # 30+ agent definitions (gold mine)
everything-claude-code/the-longform-guide.md  # Comprehensive guide
everything-claude-code/commands/       # Custom command patterns
everything-claude-code/research/       # Claude Code internals research
```

## Porting Priority: P0

This is the most immediately actionable reference. The agent/skill/hook/rule patterns
are already partially implemented in RUSVEL. Study the community patterns to refine them.
