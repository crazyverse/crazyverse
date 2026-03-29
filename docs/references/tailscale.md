# Tailscale — Reference Analysis

> WireGuard-based mesh VPN. Zero-config, NAT traversal, P2P encrypted networking.

## Architecture

```
cmd/tailscaled/         Main daemon binary
control/                Control plane client
  controlclient/        Communication with coordination server
  controlhttp/          HTTP transport for control protocol
  controlbase/          Noise protocol handshake
wgengine/               WireGuard engine wrapper
  magicsock/            NAT traversal socket (STUN, DERP, direct)
  router/               OS network routing (iptables, netlink, etc.)
derp/                   DERP relay server (encrypted relay when P2P fails)
  derphttp/             HTTP-based DERP transport
  derpserver/           DERP server implementation
net/                    Network utilities
  netcheck/             Connectivity probing
  stun/                 STUN client
  dns/                  DNS resolver
  interfaces/           OS network interface detection
types/                  Shared types
  key/                  WireGuard + Node keys
  netmap/               Network map (all nodes, their addresses, keys)
ipn/                    "IP Networking" — local state machine
client/                 Client library (Go API)
```

### How Mesh Networking Works

```
1. Node boots → contacts coordination server
2. Coordination server returns NetMap (all peers, their keys, endpoints)
3. magicsock tries to establish direct WireGuard connection:
   a. STUN to discover public IP/port
   b. Try direct UDP connection
   c. If NAT traversal fails → fall back to DERP relay
4. All traffic is WireGuard-encrypted end-to-end
5. DERP relay only sees encrypted packets (can't read content)
```

### Key Abstractions

| Concept | Tailscale | RASHK Equivalent |
|---------|-----------|------------------|
| NetMap | `types/netmap` — all peers + routes | `MeshPort::discover()` result |
| MagicSock | `wgengine/magicsock` — NAT traversal | `MeshPort` transport layer |
| DERP | `derp/` — relay when P2P fails | Relay adapter for MeshPort |
| Node Key | `types/key` — identity keypair | `IdentityPort` keypair |
| Control | `control/` — coordination server | Optional coordination service |
| Engine | `wgengine/` — WireGuard wrapper | Not needed (we use QUIC, not WireGuard) |

## What to Extract for RASHK

### 1. NAT Traversal Strategy → `rashk-mesh`
magicsock's approach: try direct → STUN → DERP fallback.
- Study: `wgengine/magicsock/magicsock.go` (main socket)
- Study: `wgengine/magicsock/derp.go` (DERP integration)
- RASHK: implement similar fallback chain in QUIC transport

### 2. DERP Relay → `rashk-mesh` relay adapter
When P2P fails, DERP relays encrypted packets. Cheap to run, stateless.
- Study: `derp/derp.go` (protocol), `derpserver/` (server)
- RASHK: build a DERP-like relay for QUIC fallback

### 3. Coordination Server Pattern → Optional `rashk-mesh` coordinator
Tailscale's control plane distributes NetMap. Open-source alternative: Headscale.
- Study: `control/controlclient/auto.go` (state machine)
- RASHK: optional coordination service, but also support mDNS for LAN

### 4. Peer Discovery → `MeshPort::discover()`
NetMap contains all peer addresses, public keys, endpoints.
- Study: `types/netmap/netmap.go`
- RASHK: `Peer` struct already has node_id, addresses, capabilities

### 5. Key Management → `rashk-identity`
Tailscale uses separate node keys (WireGuard) and machine keys (auth).
- Study: `types/key/` (key types, serialization)
- RASHK: Ed25519 keypair already in `rashk-identity`

## Key Files to Study

```
wgengine/magicsock/magicsock.go      # The magic — NAT traversal socket
wgengine/magicsock/derp.go           # DERP relay integration
derp/derp.go                          # DERP protocol (relay)
control/controlclient/auto.go         # Control plane state machine
types/netmap/netmap.go                # Network map (peer list)
types/key/                            # Key types (node, machine, disco)
net/netcheck/netcheck.go              # Connectivity probing
```

## Porting Priority: P2

Focus on: NAT traversal strategy, DERP-like relay, peer discovery protocol.
Phase 3 work — mesh networking is not needed until local-first is solid.
