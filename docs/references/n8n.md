# n8n — Reference Analysis

> Open-source workflow automation. 400+ integrations, visual DAG builder, self-hostable.

## Architecture

```
packages/
  workflow/        Core types: INode, IWorkflow, IConnection, IExecuteData
  core/            Execution engine: WorkflowExecute, credential management
  cli/             Server: Express app, webhook handling, queue worker
  frontend/        Vue.js visual workflow builder
  nodes-base/      400+ built-in node implementations
  node-dev/        SDK for building custom nodes
  @n8n/            Internal packages (config, permissions, etc.)
```

### Key Abstractions

| Concept | n8n Type | RASHK Equivalent |
|---------|----------|------------------|
| Workflow | `IWorkflowBase` (JSON DAG) | `flow-engine` DAG (petgraph) |
| Node | `INodeType` (execute fn + properties) | Flow node types (code, condition, agent) |
| Connection | `IConnection` (from/to/index) | petgraph edges |
| Trigger | `ITriggerFunctions` (poll/webhook) | `rusvel-webhook` + `rusvel-cron` |
| Credential | `ICredentialType` (encrypted store) | `rusvel-auth` |
| Execution | `IRunExecutionData` (full state) | Flow execution context |

### Execution Model

```
Trigger fires
  → WorkflowExecute.run()
    → For each node in topological order:
      → Resolve input data from connections
      → Call node.execute(inputData)
      → Pass output to connected nodes
    → Store execution result
```

Key pattern: **each node receives `IExecuteFunctions`** — a facade that provides:
- `getInputData()` — data from previous nodes
- `getCredentials()` — decrypted secrets
- `helpers.request()` — HTTP client
- `helpers.getBinaryDataBuffer()` — binary handling

## What to Extract for RASHK

### 1. Node Type System → `flow-engine` node types
n8n's `INodeType` interface is clean and extensible.
- `description: INodeTypeDescription` — name, inputs, outputs, properties (UI schema)
- `execute(this: IExecuteFunctions): Promise<INodeExecutionData[][]>`
- RASHK: extend flow-engine's 3 node types with this pattern

### 2. Credential Encryption → `rusvel-auth`
n8n encrypts credentials at rest with a unique key per installation.
- Study: `packages/core/src/CredentialTypes.ts`
- RASHK: `IdentityPort::sign()` for credential encryption

### 3. Webhook Trigger → `rusvel-webhook`
n8n registers webhook URLs per workflow, routes incoming HTTP to workflow execution.
- Study: `packages/cli/src/webhooks/`
- RASHK: already has webhook → job queue pattern

### 4. Visual Workflow Builder → `frontend/`
Vue.js canvas with drag-and-drop nodes, connection drawing, execution visualization.
- Study: `packages/frontend/src/components/canvas/`
- RASHK: `WorkflowBuilder.svelte` component (already started)

### 5. Queue-based Execution → `rusvel-jobs`
n8n supports Bull queue for distributed execution.
- Study: `packages/cli/src/scaling/`
- RASHK: job queue already exists, extend with workflow context

## Key Files to Study

```
packages/workflow/src/Interfaces.ts          # Core type definitions
packages/workflow/src/Workflow.ts             # Workflow class
packages/core/src/WorkflowExecute.ts         # DAG execution engine
packages/core/src/NodeExecuteFunctions.ts    # What nodes can do
packages/nodes-base/nodes/                    # Node implementations (patterns)
packages/cli/src/webhooks/                    # Webhook trigger handling
```

## Porting Priority: P1

Focus on: Node type interface, DAG execution improvements, visual builder patterns.
