# P7: Multi-Chain Wallet Architecture in Rust

**Source:** Perplexity research, March 2026
**Status:** DONE
**Decision impact:** D4 — DID/identity, D9 — multi-chain

## Key Findings

- **Derivation model:** One mnemonic, multiple derivation paths — seed-level shared, account-level separate
- **Solana:** `m/44'/501'/{index}'/0'` (Ed25519, SLIP-0010 derivation)
- **EVM:** `m/44'/60'/0'/0/{index}` (secp256k1, BIP-32 derivation)
- **TON:** `m/44'/607'/0'/0'/0'/0'` (Ed25519, but TON-specific path + wallet contract addressing)
- **Key compatibility:** Solana and TON both use Ed25519 but NOT interchangeable — different paths, different addresses
- **TON specifics:** Adds wallet contract layer on top of bare public key (unlike Solana's direct pubkey-as-address)
- **Brave Wallet:** Built into Chromium (not extension), BIP-32/39/43/44, multi-chain
- **Rust crates:** bip39, ed25519-dalek, secp256k1. Need SLIP-0010 for Ed25519 HD derivation

## Recommended Rust Layout

```
seed module → hd module (two derivers) → chains module → accounts module → storage module
```

## Secure Storage

- OS keychain for wrapping key
- Ciphertext in app storage
- HSM/Ledger for signing (optional)

## RASHK Implications

- One mnemonic, two Ed25519 namespaces (Solana + TON) — clean architecture
- IdentityPort manages the seed, chain-specific adapters derive keys per chain
- SLIP-0010 is required for Ed25519 HD derivation (not standard BIP-32)
- TON's wallet contract layer adds complexity vs Solana's direct pubkey model
- Storage: OS keychain wrapping key + encrypted app storage is the right default
