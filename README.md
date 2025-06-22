# axum-ethers

A simple Rust web API built with [Axum](https://github.com/tokio-rs/axum) and [ethers-rs](https://github.com/gakonst/ethers-rs) to fetch Ethereum ETH and ERC-20 token balances using Alchemy’s JSON-RPC API.

## Features

- Fetch ETH balance for any Ethereum address
- Fetch ERC-20 token balances via Alchemy API
- Built with async Rust for high performance and safety

## Getting Started

### Prerequisites

- Rust (recommended latest stable)
- An [Alchemy API key](https://dashboard.alchemy.com/) with Ethereum Mainnet access

### Setup

1. Clone the repository

   ```bash
   git clone https://github.com/RafaelPil/axum-ethers.git
   cd axum-ethers
   ```
2. Create a .env file in the project root with your Alchemy API key:

   ```bash
   ETH_RPC_URL=https://eth-mainnet.g.alchemy.com/v2/YOUR_ALCHEMY_API_KEY
   ```
3. Build and run the server:
   ```bash
   cargo run
   ```
4. The server will run on http://localhost:3000

### Usage

- Get ETH balance for an address:
   ```bash
   curl http://localhost:3000/eth/balance/address
   ```

- Get ERC-20 token balances for an address:
   ```bash
   curl curl http://localhost:3000/eth/tokens/address
   ```

### Project Structure
- main.rs: app entry point, routing setup
- handlers.rs: request handlers for balances
- .env: environment variables (not committed to Git)

### Notes
- Make sure .env is included in .gitignore to keep your API keys private.
- This project uses Alchemy’s API; ensure your API key has the necessary permissions.
- Extend or customize the token balance parsing to fit your needs.