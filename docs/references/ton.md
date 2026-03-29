# TON Blockchain — Reference Analysis

> The Open Network. Multi-blockchain, ADNL networking, TVM smart contracts.

## Architecture

TON is a C++ monorepo:

```
adnl/               Abstract Datagram Network Layer — P2P networking protocol
  adnl-peer.cpp     Peer management, message routing
  adnl-channel.cpp  Encrypted channels between peers
  adnl-node.cpp     Node identity and routing
catchain/            Consensus protocol (Byzantine fault-tolerant)
crypto/              Cryptographic primitives + smart contracts
  vm/               TON Virtual Machine (TVM) — stack-based VM for contracts
  block/            Block structure, validators, sharding
  smartcont/        Standard smart contract code (wallets, multisig, etc.)
  func/             FunC compiler (smart contract language)
  fift/             Fift language (low-level assembler for TVM)
dht/                 Distributed Hash Table for peer discovery
  dht-server/       DHT node implementation
lite-client/         Lightweight client for querying blockchain state
validator/           Full validator node
overlay/             Overlay networks (pub/sub on top of ADNL)
rldp/                Reliable Large Datagram Protocol (over ADNL)
storage/             Torrent-like file storage on TON
tl/                  Type Language schema definitions (like protobuf)
emulator/            Smart contract emulator for testing
```

### Key Concepts

| Concept | TON Implementation | RASHK Equivalent |
|---------|-------------------|------------------|
| ADNL | P2P encrypted UDP with abstract addresses | `MeshPort` — P2P messaging |
| DHT | Kademlia-based peer discovery | `MeshPort::discover()` |
| TVM | Stack-based VM for smart contracts | `ModuleRuntime` (WASM instead) |
| Wallet | Smart contract with keypair | `IdentityPort` + `PaymentPort` |
| Overlay | Topic-based pub/sub over ADNL | `EventBus` (cross-node) |
| TL Schema | Interface description language | WIT (WASM Interface Types) |

### ADNL Protocol (most relevant for RASHK)

```
1. Each node has an abstract address (hash of public key)
2. Messages are encrypted with receiver's public key
3. Transport: UDP datagrams, encrypted channel establishment
4. Routing: direct P2P or via DHT-based relay
5. Higher protocols: RLDP (reliable), overlay (pub/sub)
```

## What to Extract for RASHK

### 1. ADNL Abstract Addressing → `rashk-mesh`
ADNL uses hash-of-public-key as node address — location-independent identity.
- Study: `adnl/adnl-node-id.cpp`, `adnl-peer.cpp`
- RASHK: `NodeId` is already UUID — could be hash of Ed25519 pubkey instead

### 2. Encrypted Channel Pattern → `rashk-mesh`
ADNL establishes encrypted channels between peers, then multiplexes.
- Study: `adnl/adnl-channel.cpp`
- RASHK: QUIC already provides this, but the channel abstraction is useful

### 3. DHT for Peer Discovery → `rashk-mesh`
Kademlia DHT for finding peers by their abstract address.
- Study: `dht/dht-server/`, `dht/dht-query.cpp`
- RASHK: combine mDNS (LAN) + DHT (WAN) for discovery

### 4. Wallet Smart Contracts → `PaymentPort`
TON wallets are smart contracts — programmable payment logic.
- Study: `crypto/smartcont/wallet-v4.fif` (standard wallet)
- RASHK: PaymentPort abstracts this — adapter decides if Solana or TON

### 5. Overlay Networks → `EventBus` cross-node
Topic-based pub/sub over ADNL — nodes subscribe to event streams.
- Study: `overlay/overlay.cpp`
- RASHK: `EventBus::subscribe(pattern)` extended to cross-node delivery

### 6. TL Schema → WIT interface definitions
TON uses TL (Type Language) to define all protocol messages.
- Study: `tl/generate/`, `tl/tl/`
- RASHK: WIT already serves this role for WASM modules

## Key Files to Study

```
adnl/adnl-peer.cpp              # Peer management and message routing
adnl/adnl-channel.cpp           # Encrypted channel establishment
adnl/adnl-node-id.cpp           # Abstract addressing (hash of pubkey)
dht/dht-query.cpp                # DHT peer discovery
crypto/vm/                       # TVM (contract execution model)
crypto/smartcont/                 # Standard contract code
overlay/overlay.cpp               # Pub/sub overlay network
tl/generate/                      # Protocol schema generation
```

## Porting Priority: P3

Focus on: ADNL abstract addressing concept, DHT discovery, overlay pub/sub pattern.
Phase 4-5 work. TON is a reference for networking patterns, not a direct dependency.
Solana (Agave) is primary for payments; TON is secondary or alternative.
