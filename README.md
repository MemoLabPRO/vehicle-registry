# Vehicle Registry — Solana Program

A foundational on-chain vehicle title registry built with Anchor on Solana.  
Every vehicle is a PDA derived from its VIN — immutable, verifiable, transferable.

## Problem

In peer-to-peer vehicle sales, there is no trustless way to verify ownership before payment.  
Buyers rely on paper titles that can be forged, duplicated, or delayed.  
This program puts vehicle ownership on-chain, making it queryable, transferable, and composable with sale contracts.

## Solution

A Solana program that registers vehicles as Program Derived Accounts (PDAs) using the VIN as a seed.  
Any sale contract — including a P2P HST-compliant sale contract — can call `transfer_title` via CPI to update ownership atomically with payment.

## Program ID (Devnet)

```
3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3
```

🔍 [View on Solana Explorer](https://explorer.solana.com/address/3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3?cluster=devnet)

## Instructions

| Instruction | Who calls it | What it does |
|---|---|---|
| `register_vehicle` | Vehicle owner | Creates a PDA account for a VIN, sets initial owner |
| `update_status` | Current owner | Sets status: `Active`, `ForSale`, or `Stolen` |
| `transfer_title` | Owner or sale contract via CPI | Transfers ownership to a new wallet, resets status to `Active` |

## Account Structure

```rust
pub struct VehicleRecord {
    pub vin: [u8; 17],        // Vehicle Identification Number
    pub owner: Pubkey,         // Current owner wallet
    pub make: String,          // e.g. "Honda"
    pub model: String,         // e.g. "Civic"
    pub year: u16,             // e.g. 2021
    pub status: VehicleStatus, // Active | ForSale | Stolen
    pub registered_at: i64,    // Unix timestamp
    pub bump: u8,              // PDA bump seed
}
```

## PDA Derivation

```
seeds = ["vehicle", vin_bytes]
program_id = 3CgTkABSuEUC6C6U5zpuzB2SRhggThSHsVLe3NvxKYY3
```

Given any VIN, anyone can derive the on-chain address deterministically — no indexer needed.

## Why This Matters for Solana

This program is a **primitive** — a building block.  
It is designed to be composed with other programs via CPI:

- A P2P sale contract can call `transfer_title` atomically with SOL payment
- A financing contract can lock status while a loan is active
- An inspection oracle can write certified reports to the same account

One registry. Many programs on top.

## Local Development

```bash
# Install dependencies
yarn install

# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy
```

## Tests

```
✔ Registra un vehículo
✔ Cambia estado a ForSale  
✔ Transfiere el título a nuevo propietario

3 passing (1s)
```

## Tech Stack

- Rust · Anchor 0.32.1
- Solana CLI 3.0.15
- TypeScript · Mocha (tests)

## Author

Guillermo (Memo) — Solana Bootcamp · WayLearn Latam  
GitHub: [@MemoLabPRO](https://github.com/MemoLabPRO)
