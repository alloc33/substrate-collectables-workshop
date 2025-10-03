# Substrate Kitties NFT Marketplace

A Substrate pallet implementing a complete NFT marketplace for digital collectibles called "Kitties". Each kitty is a unique non-fungible token with randomly generated DNA, ownership tracking, and marketplace functionality.

## Features

- **NFT Creation**: Generate unique kitties with 32-byte DNA signatures
- **Ownership Management**: Track and transfer ownership between accounts
- **Marketplace**: Set prices and trade kitties with built-in escrow
- **Collection Limits**: Bounded collections to prevent storage bloat
- **Event System**: Comprehensive event emissions for all marketplace activities

## Core Functionality

- `create_kitty()` - Mint new kitties with unique DNA
- `transfer()` - Transfer ownership between accounts
- `set_price()` - List kitties for sale or remove from market
- `buy_kitty()` - Purchase kitties with price validation

## Storage

- Tracks total kitty count and individual kitty metadata
- Maps accounts to their owned kitty collections (max 100 per account)
- Stores pricing information for marketplace listings

## Development

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

### Format & Lint
```bash
cargo +nightly fmt
cargo +nightly clippy
```

## Dependencies

Built with [Polkadot SDK](https://github.com/paritytech/polkadot-sdk) using the latest FRAME v2 architecture.
