# Anchor NFT Marketplace

This repository contains a Solana program built with **Anchor** for an NFT marketplace. The program allows users to initialize a marketplace, list NFTs, purchase NFTs, and delist NFTs, with secure handling of token transfers, fees, and account management.

## Overview

The **Anchor NFT Marketplace** enables users to:
- **Initialize** a marketplace with a unique name, fee structure, and treasury for collecting fees.
- **List** NFTs by transferring them to a program-owned vault and storing listing metadata.
- **Purchase** NFTs, paying the lister and marketplace fees, and transferring ownership to the buyer.
- **Delist** NFTs, returning them to the lister and cleaning up accounts to reclaim rent.

The program uses **Anchor** for Solana development and integrates with the **SPL Token Program** (including Token2022) for NFT handling. It leverages Program Derived Addresses (PDAs) for secure account management and uses the **Metaplex Metadata Program** for NFT collection verification.

## Program ID
The program ID is: `HGYmo6dFcDhvzyaGHwLYUkSWqZZp8BoEHDuJnMobMWkP`

## Prerequisites

To build, deploy, or interact with this program, you need:
- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
- **Solana CLI**: Install with `sh -c "$(curl -sSfL https://release.solana.com/v1.18.17/install)"`.
- **Anchor**: Install using `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`.
- **Node.js**: For running tests (optional, version 16+ recommended).
- A Solana wallet with some SOL for deployment and testing (e.g., Phantom or Solana CLI wallet).
- **Yarn** or **npm** for managing dependencies in the test suite.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd anchor-nft-marketplace
   ```

2. **Install Dependencies**:
   ```bash
   yarn install
   ```
   Or, if using npm:
   ```bash
   npm install
   ```

3. **Build the Program**:
   ```bash
   anchor build
   ```

4. **Deploy to Devnet**:
   Ensure your Solana CLI is configured for devnet:
   ```bash
   solana config set --url devnet
   anchor deploy
   ```

5. **Run Tests** (optional):
   ```bash
   anchor test
   ```

## Program Structure

The program is organized into modules:
- **constants**: Defines constants used across the program.
- **error**: Custom error types for the marketplace.
- **instructions**: Core logic for `initialize_marketplace`, `list`, `delist`, and `purchase_nft`.
- **state**: Data structures for `Marketplace` and `Listing` accounts.

### Instructions

#### 1. Initialize Marketplace
Creates a new marketplace with a unique name, admin, fee structure, and treasury.
- **Accounts**:
  - `admin`: Signer who creates the marketplace.
  - `marketplace`: PDA for marketplace state (`seeds=[b"marketplace", name.as_bytes()]`).
  - `treasury`: PDA for collecting fees (`seeds=[b"treasury", marketplace.key()]`).
  - `rewards_mint`: PDA for reward tokens (`seeds=[b"rewards", marketplace.key()]`).
- **Parameters**:
  - `name`: Unique marketplace name.
  - `fee`: Fee percentage in basis points (e.g., 250 = 2.5%).
- **Behavior**: Initializes the marketplace with the provided configuration and bumps for PDAs.

#### 2. List NFT
Lists an NFT by transferring it to a program-owned vault and creating a listing PDA.
- **Accounts**:
  - `lister`: Signer who owns the NFT.
  - `marketplace`: Marketplace PDA for validation.
  - `lister_mint`: NFT mint address.
  - `collection_mint`: Mint of the NFT collection.
  - `lister_ata`: Lister's associated token account (ATA) holding the NFT.
  - `listing`: PDA for listing metadata (`seeds=[marketplace.key(), lister_mint.key()]`).
  - `vault`: Program-owned ATA for holding the NFT.
  - `metadata`: Metaplex metadata account for the NFT.
  - `master_edition`: Metaplex master edition account.
- **Parameters**:
  - `price`: Listing price in lamports.
- **Behavior**:
  - Verifies the NFT belongs to the specified collection (via Metaplex metadata).
  - Transfers the NFT from `lister_ata` to `vault`.
  - Initializes the `listing` PDA with price and bump.

#### 3. Purchase NFT
Purchases an NFT, transferring it to the buyer, paying the lister, and collecting marketplace fees.
- **Accounts**:
  - `buyer`: Signer purchasing the NFT.
  - `lister`: Original lister's account (receives payment).
  - `marketplace`: Marketplace PDA for fee and bump validation.
  - `treasury`: PDA for collecting marketplace fees.
  - `nft_mint`: NFT mint address.
  - `buyer_nft_ata`: Buyer's ATA for receiving the NFT.
  - `vault`: Program-owned ATA holding the NFT.
  - `listing`: Listing PDA to be closed.
- **Behavior**:
  - Calculates marketplace fees based on the listing price and fee percentage.
  - Transfers fees to `treasury`.
  - Transfers the remaining amount to `lister`.
  - Transfers the NFT from `vault` to `buyer_nft_ata`.
  - Closes the `vault` and `listing` accounts, refunding rent to the lister.

#### 4. Delist NFT
Allows the lister to reclaim their NFT and close the listing.
- **Accounts**:
  - `lister`: Signer who listed the NFT.
  - `marketplace`: Marketplace PDA for validation.
  - `lister_mint`: NFT mint address.
  - `lister_ata`: Lister's ATA to receive the NFT.
  - `listing`: Listing PDA to be closed.
  - `vault`: Program-owned ATA holding the NFT.
- **Behavior**:
  - Transfers the NFT from `vault` to `lister_ata`.
  - Closes the `listing` PDA, refunding rent to the lister.
  - Closes the `vault` account manually using the SPL Token Program.

## Security Considerations
- **PDA Security**: All PDAs use proper seeds and bumps to ensure uniqueness and prevent unauthorized access.
- **Account Ownership**: The `vault` account is owned by the `listing` PDA, ensuring only authorized instructions can transfer or close it.
- **Collection Verification**: The `list` instruction verifies NFT collection membership using Metaplex metadata to prevent listing invalid NFTs.
- **Rent Management**: Accounts like `listing` and `vault` are closed when no longer needed, refunding rent to the appropriate party.
- **Token Program**: Supports both SPL Token and Token2022 via `token_interface` for compatibility.

## Testing
To test the program:
1. Set up a local Solana validator:
   ```bash
   solana-test-validator
   ```
2. Run the test suite:
   ```bash
   anchor test
   ```
The test suite covers:
- Marketplace initialization.
- Listing and delisting NFTs.
- Purchasing NFTs with fee distribution.
- Error cases (e.g., invalid collection, unauthorized access).

## Usage Example
1. **Initialize a Marketplace**:
   Use the `initialize_marketplace` instruction with a unique name and fee (e.g., 250 basis points = 2.5%).
2. **List an NFT**:
   Call `list` with the NFT mint, collection mint, and desired price. Ensure the NFT is in the lister’s ATA and part of a verified collection.
3. **Purchase an NFT**:
   Use `purchase_nft` to buy an NFT, specifying the buyer’s wallet and ATA. The program handles fee distribution and NFT transfer.
4. **Delist an NFT**:
   If not sold, the lister can call `delist` to reclaim the NFT and close accounts.

## Contributing
Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.