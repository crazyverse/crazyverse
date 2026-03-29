# RASHK — The Universe Model

> Model the universe as entities and services.
> Discoverable. Agentic. Contractual. Transparent.

## The Ultimate Goal

The world already has computational power. What it doesn't have is a **universal protocol for entities to find each other, agree on terms, do work, and exchange value** — without middlemen, without opacity, without lock-in.

RASHK models this:

```
ENTITY ←──discovers──→ ENTITY
   │                      │
   └──── CONTRACT ────────┘
            │
      ┌─────┼──────┐
      │     │      │
    AGENT  RULES  PAYMENT
   (does)  (code) (web3)
```

**Anyone who has a product or service** — a freelancer, a business, a machine, an AI agent — registers as an entity, publishes what they offer, and becomes discoverable.

**Anyone who needs a product or service** — a person, a business, another agent — searches, filters, evaluates, and contracts.

**The contract is on-chain.** Terms are code. Execution is agents. Payment is automatic. Disputes are transparent. No platform takes 30%. No intermediary decides who can participate.

---

## Core Concepts

### Entity

Everything is an entity. A person, a business, an AI agent, a machine, a module, a service endpoint. An entity:

- **Has identity** — cryptographic, self-sovereign (DID + keypair)
- **Is discoverable** — registered, searchable, filterable
- **Is agentic** — can act autonomously (AI-powered or rule-based)
- **Has reputation** — on-chain history of contracts fulfilled
- **Has capabilities** — what it can do, what it offers
- **Lives somewhere** — a node on the mesh, a cloud endpoint, a chain address

An entity is NOT an account on someone's platform. It's a sovereign participant in a network.

### Service

What an entity offers to the world. A service:

- **Has a definition** — what it does, inputs, outputs, SLA
- **Has a price** — fixed, per-use, subscription, auction, negotiable
- **Is versioned** — evolves over time, backwards-compatible
- **Is composable** — one service can use other services
- **Is executable** — an agent or automation can invoke it directly

Examples:
- A freelancer offers "logo design" — input: brief, output: artifact, price: 500 USDC
- A machine offers "GPU compute" — input: job spec, output: result, price: per-minute
- An AI agent offers "code review" — input: PR diff, output: review, price: 0.10 USDC
- A module offers "invoicing" — input: line items, output: invoice + payment link
- A data provider offers "market data" — input: query, output: dataset, price: per-call

### Discovery

How entities find each other. Not a search engine — a **protocol**:

- **Local mesh** — mDNS discovers entities on your network
- **Public mesh** — DHT/registry discovers entities globally
- **Agent-assisted** — "find me a Rust developer who charges under $100/hr and has >90% reputation"
- **Chain-indexed** — on-chain service registrations are queryable
- **Filtered** — by capability, price, reputation, location, availability

Discovery is not a centralized marketplace. It's a network property. Every node participates in discovery by announcing what it offers and relaying queries.

### Contract

How two entities agree to exchange value. A contract:

- **Is code** — Solana program / smart contract
- **Has terms** — what, when, how much, what if failure
- **Is enforceable** — escrow locks funds, release on delivery
- **Is transparent** — anyone can audit the contract on-chain
- **Is composable** — a contract can reference sub-contracts

The contract lifecycle:
```
PROPOSE → NEGOTIATE → AGREE → ESCROW → EXECUTE → DELIVER → VERIFY → PAY → RATE
```

Agents handle most of this. A human approves the proposal and the final payment. Everything in between is autonomous.

### Agent (as first-class economic actor)

In RASHK, an agent isn't a chatbot. It's an **economic actor** that can:

- **Discover** services and entities on the network
- **Evaluate** options using AI reasoning
- **Negotiate** terms with other agents
- **Execute** work by calling tools, services, and modules
- **Transact** by signing and submitting chain transactions
- **Report** results back to its owner
- **Learn** from outcomes to improve future decisions

An agent acts on behalf of an entity. Your agent is you — it has your identity, your wallet, your permissions, your preferences. It does what you would do, but faster, 24/7, across every service on the network.

### Payment (Web3-native)

Money movement in RASHK is:

- **On-chain** — Solana transactions, SPL tokens
- **Programmable** — escrow, milestone-based, streaming, subscription
- **Transparent** — every payment is auditable
- **Instant** — Solana finality in seconds
- **Composable** — payment triggers downstream events
- **Agent-initiated** — agents pay on behalf of entities

No Stripe. No PayPal. No bank wire. No "net 30." Code sends money when conditions are met.

---

## How It All Connects

```
┌──────────────────────────────────────────────────────────────────┐
│                        THE RASHK UNIVERSE                        │
│                                                                  │
│   ┌──────────┐     discovers      ┌──────────┐                  │
│   │ Entity A │ ──────────────────→│ Entity B │                  │
│   │ (buyer)  │                    │ (seller) │                  │
│   │          │                    │          │                  │
│   │ Agent A  │     negotiates     │ Agent B  │                  │
│   │    ↕     │ ←────────────────→ │    ↕     │                  │
│   │ Wallet A │                    │ Wallet B │                  │
│   └────┬─────┘                    └────┬─────┘                  │
│        │                               │                         │
│        │         ┌──────────┐          │                         │
│        └────────→│ Contract │←─────────┘                         │
│                  │ (on-chain)│                                    │
│                  │           │                                    │
│                  │  Terms    │                                    │
│                  │  Escrow   │                                    │
│                  │  Deadline │                                    │
│                  └─────┬─────┘                                   │
│                        │                                         │
│              ┌─────────┼─────────┐                               │
│              │         │         │                                │
│          ┌───▼───┐ ┌───▼───┐ ┌──▼────┐                          │
│          │Execute│ │Verify │ │  Pay  │                           │
│          │(agent)│ │(chain)│ │(chain)│                           │
│          └───────┘ └───────┘ └───────┘                           │
│                                                                  │
│   Runs on: your machine, your mesh, your keys, your rules       │
└──────────────────────────────────────────────────────────────────┘
```

---

## The Value Layers

### Layer 0: Compute
Any machine with RASHK installed is compute. Your laptop, your server, a rented GPU. The mesh connects them. The runtime executes on them. This is the substrate.

### Layer 1: Entity
Everything registers as an entity with identity, capabilities, and discoverability. The entity is the atom of the universe.

### Layer 2: Service
Entities publish services — what they can do. Services are typed, priced, versioned, composable. The service is the verb of the universe.

### Layer 3: Contract
Entities agree on terms via on-chain contracts. Escrow, milestones, SLAs, dispute resolution. The contract is the trust layer.

### Layer 4: Agent
AI agents act on behalf of entities. They discover, negotiate, execute, transact. The agent is the autonomy layer.

### Layer 5: Economy
Token flows, reputation accumulates, governance evolves. The economy is the incentive layer that makes the whole thing self-sustaining.

```
Economy    ── incentives make it grow ──────────────────── self-sustaining
Agent      ── AI makes it autonomous ──────────────────── 24/7 operation
Contract   ── code makes it trustless ─────────────────── no middlemen
Service    ── APIs make it composable ─────────────────── infinite combinations
Entity     ── identity makes it sovereign ─────────────── you own yourself
Compute    ── mesh makes it unstoppable ───────────────── runs anywhere
```

Every layer is optional except Compute + Entity. You can run RASHK as a solo tool on your laptop (Compute + Entity + Service for yourself). Or you can participate in the full economy. Same binary, same ports, different adapter wiring.

---

## What Exists Today vs. This Vision

| Concept | Today's world | RASHK |
|---------|--------------|-------|
| Discovery | Google, App Store, word of mouth | Network protocol, agent-assisted |
| Agreement | Emails, PDFs, verbal | On-chain contract, code-enforced |
| Payment | Stripe, PayPal, wire transfer | Solana, instant, programmable |
| Execution | Human does the work | Agent executes, human approves |
| Trust | Platform reviews, LinkedIn endorsements | On-chain reputation, verifiable history |
| Disputes | Lawyers, chargebacks, support tickets | Contract code, transparent arbitration |
| Platform cut | 15-30% (App Store, Upwork, Fiverr) | 0% or minimal network fee |
| Data ownership | Platform owns your data | You own everything, local-first |
| Availability | Business hours, time zones | Agent runs 24/7 on your node |
| Composability | Zapier duct tape, API wrangling | Native service composition via ports |

---

## From RUSVEL to RASHK to Universe

```
RUSVEL (proved)          RASHK (building)         UNIVERSE (goal)
──────────────           ──────────────           ──────────────
Departments              Modules + Apps           Entities + Services
Local tools              Network of nodes         Global mesh
Agent chat               Agent execution          Agent economy
No payments              Chain transactions       Contract economy
One user                 Team mesh                Open marketplace
Port traits              Port traits              Port traits (same!)
```

The port traits don't change. The adapter wiring scales from "my laptop" to "global economic network." That's the whole bet.

---

## What This Means for Phase 0

The domain model (docs/design/domain-model.md) catalogs the primitives.
This document defines why they exist and how they compose into an economy.

The ADRs, port designs, and research should now be evaluated against this question:

**"Does this design decision work when entities are discovering each other, contracting on-chain, and agents are executing autonomously across a global mesh?"**

If yes, proceed. If no, redesign.

---

*Created: 2026-03-29*
*This is the north star. Everything else serves it.*
