# Supabase — Reference Analysis

> Open-source Firebase alternative. Postgres + Auth + Realtime + Storage + Edge Functions.

## Architecture

Supabase is a **composition of open-source tools**, not a monolith:

```
apps/
  studio/          Dashboard UI (Next.js) — admin panel, SQL editor, table editor
  docs/            Documentation site
  www/             Marketing site
packages/
  pg-meta/         Postgres introspection API (schema, tables, roles, extensions)
  ai-commands/     AI-powered SQL generation
  ui/              Shared UI component library
  ui-patterns/     Higher-level UI patterns
```

### Core Services (separate repos, composed via Docker)

| Service | Repo | What it does |
|---------|------|-------------|
| **PostgREST** | postgrest/postgrest | Auto-generates REST API from Postgres schema |
| **GoTrue** | supabase/gotrue | Auth: email/password, OAuth, magic link, phone |
| **Realtime** | supabase/realtime | Elixir server: Postgres CDC → WebSocket broadcast |
| **Storage** | supabase/storage-api | S3-compatible object storage with RLS |
| **pg_graphql** | supabase/pg_graphql | GraphQL API from Postgres (as PG extension) |
| **Supavisor** | supabase/supavisor | Connection pooler (Elixir, PgBouncer replacement) |

## What to Extract for RASHK

### 1. Row Level Security (RLS) Pattern → `rashk-store`
Supabase's killer feature: security policies defined in SQL, enforced at DB level.
- Study: `pg-meta` package for schema introspection
- RASHK equivalent: capability-based access control on `Store` records

### 2. Realtime CDC → `rashk-core::EventBus`
Postgres logical replication → WebSocket events.
- Study: `supabase/realtime` repo (Elixir)
- RASHK equivalent: `EventBus::subscribe()` + CRDT sync

### 3. Auto-generated REST API → `rusvel-api`
PostgREST turns schema into API. We already do something similar with department routes.
- Study: PostgREST source for URL → SQL translation
- RASHK equivalent: `Store::query()` exposed as REST

### 4. Storage API → `rashk-core::BlobStore`
S3-compatible with RLS policies on buckets/objects.
- Study: `supabase/storage-api` (Node.js)
- RASHK equivalent: `BlobStore` port trait

### 5. Dashboard/Studio → `frontend/`
Table editor, SQL runner, schema viewer — we have `RusvelBase` already.
- Study: `apps/studio/` for UI patterns
- RASHK equivalent: `/database/*` routes in frontend

## Key Files to Study

```
packages/pg-meta/src/          # Postgres schema introspection
apps/studio/components/        # Dashboard components
apps/studio/pages/api/         # API routes
```

## Porting Priority: P1

Focus on: RLS-equivalent capability model, realtime event subscriptions, schema introspection API.
