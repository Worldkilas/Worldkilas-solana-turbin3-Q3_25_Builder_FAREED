Solana Turbine Q3 Cohort
This repository documents my journey during the Solana Turbine Q3 Cohort. It includes various experiments and projects built with Rust and TypeScript, focusing on Solana development and related tooling.

Overview
The repo is organized into several major parts:

prereqs/
Contains preliminary code and scripts for setting up the Solana environment.

prereqs-rust/: Rust-based prerequisites such as keypair management, airdrop handling, and more. (See prereqs/prereqs-rust/Cargo.toml)
prereqs-ts/: TypeScript scripts for interacting with Solana, including enrollment and key generation tasks.
solana-starter/
Holds the core Solana programs and client code for both Rust (rs) and TypeScript (ts). It features:

rs/: Rust implementations and on-chain program logic.
ts/: Scripts to interact with deployed programs and utilities to manage operations (like vault deposits and withdrawals).
vaults/
Contains the implementation of various vault programs, migration scripts, and tests. These are used to manage assets on Solana, from token transfers to NFT deposits/withdrawals.

Features
Rust Programs & Client
Use the Solana SDK to write programs and on-chain logic. The Rust modules integrate with solana-sdk for on-chain operations and tests.

TypeScript Tools
Client-side scripts run with ts-node to interact with Solana clusters and deployed programs, using modern libraries from @solana/web3.js.

Anchor Integration
Some parts of the repo (especially in vaults) use Anchor for a more structured program development experience on Solana. See scripts in solana-starter/ts for available commands.

Vault Management
The vaults directory provides modules and programs for handling vaults, including deposit, withdrawal, and NFT management. Refer to Cargo.lock and wba_vault.ts for implementation details.

Getting Started
Prerequisites
Rust (latest stable version)
Node.js and Yarn for TypeScript parts
Solana CLI for cluster management
Installation
Clone the repository

Install Rust dependencies
Navigate to the Rust project directories and run:

Install Node Dependencies
In directories with a package.json (such as solana-starter/ts or vaults/package.json):

Running Tests
Rust Tests:
In the Rust directories, run:

TypeScript Tests / Scripts:
Use the provided npm scripts from the corresponding package.json. For example:

Deployment
Follow the instructions in your build scripts or deployment pipeline as outlined in the scripts section of package.json.

Project Structure
My Journey
This repository not only serves as a working project for managing various vault functionalities on Solana but also as a diary of my experiences during the Solana Turbine Q3 Cohort. Every module, script, and test reflects a learning milestone and the challenges overcome along the way.

I hope this documentation provides a clear window into my work and the evolution of the project during the cohort.
