# P9: NixOS as Appliance OS

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** Full stack feasibility (Layer 1 — OS)

## Key Findings

- **Adoption:** Used for homelab/server appliances, commercial platform bases, hardware vendor preinstalls (NovaCustom, Framework)
- **Image generation:** nixos-generators upstreamed into Nixpkgs 25.05 — ISO, QCOW2, VMDK, AMI, Docker, LXC from single config
- **Raspberry Pi:** Supported, custom images buildable. Larger than minimal embedded distros, boot time needs tuning
- **Fleet management:** Colmena and deploy-rs are modern choices. NixOps is legacy but works. nixos-anywhere for provisioning
- **Reproducibility:** Byte-identical system closures from same config, generational rollbacks

## Comparison with Alternatives

| System | Focus | Trade-off |
|--------|-------|-----------|
| NixOS | Most flexible, declarative, reproducible | Most DIY |
| Flatcar Linux | Container-optimized immutable OS | Less flexible |
| Talos | Kubernetes-only API-driven OS | Too narrow for RASHK |
| Bottlerocket | AWS container OS | Vendor-tied |

## Current State

"Off-the-shelf appliance OS on NixOS" is still emerging — you build it yourself. But the primitives (image generators, fleet tools, reproducibility) are now first-class.

## RASHK Implications

- NixOS confirmed as right choice for RashkOS (Phase 9)
- Flexible enough for laptop, Raspberry Pi, VPS, and cloud targets from one config
- Image generators are now first-class in Nixpkgs — no custom tooling needed
- Fleet management via Colmena + Git-ops is the recommended approach
- Boot time on Pi needs tuning but is workable
- The DIY nature is acceptable since RASHK needs deep customization anyway
