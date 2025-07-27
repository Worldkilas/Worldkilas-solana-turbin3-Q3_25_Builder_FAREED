# Anchor Escrow

This repository contains a Solana program built with **Anchor** for an escrow service that facilitates atomic token swaps between a maker and a taker. The program allows a maker to deposit tokens into an escrow vault, a taker to complete the swap by providing the requested tokens, and the maker to refund and reclaim their tokens if the swap is not completed.

## Overview

The **Anchor Escrow** program enables secure, trustless token swaps by locking tokens in an escrow until the swap conditions are met. Key features include:
- **Making an Escrow**: A maker initializes an escrow by depositing a specified amount of Token A and defining the amount of Token B they wish to receive in exchange.
- **Taking an Escrow**: A taker completes the swap by sending the requested amount of Token B to the maker, receiving Token A from the escrow vault, and closing the escrow accounts.
- **Refunding an Escrow**: If no taker completes the swap, the maker can reclaim their deposited Token A and close the escrow, recovering any rent paid.
- **Security Features**: Utilizes Program Derived Addresses (PDAs) for secure vault ownership, signer seeds for authorized token transfers, and account constraints to ensure valid operations.

The program integrates with the **SPL Token Program** (via `token_interface`) for token transfers and account management, ensuring compatibility with SPL tokens (including Token2022). It uses PDAs to manage the escrow account and vault securely, ensuring only authorized parties can interact with the locked tokens.



## Prerequisites

To build, deploy, or interact with this program, you need:
- **Rust**: Install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`.
- **Solana CLI**: Install with `sh -c "$(curl -sSfL https://release.solana.com/v1.18.17/install)"`.
- **Anchor**: Install using `cargo install --git https://github.com/coral-xyz/anchor anchor-cli --locked`.
- **Node.js**: For running tests (optional, version 16+ recommended).
- A Solana wallet with some SOL for deployment and testing (e.g., Phantom or Solana CLI wallet).
- **Yarn** or **npm** for managing test suite dependencies.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone <repository-url>
   cd anchor-escrow
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
   Configure Solana CLI for devnet:
   ```bash
   solana config set --url devnet
   anchor deploy
   ```

5. **Run Tests** (optional):
   ```bash
   anchor test
   ```

## Program Structure

The program is organized into the following modules:
- **constants**: Defines program-wide constants.
- **error**: Custom error types for handling failure cases.
- **instructions**: Core logic for the `make`, `take`, and `refund` instructions.
- **state**: Defines the `Escrow` account structure for storing escrow metadata.

### Data Structures
- **Escrow Account**:
  - **Fields**:
    - `seeds` (u64): Unique seed for PDA derivation.
    - `maker` (Pubkey): Public key of the maker who created the escrow.
    - `mint_a` (Pubkey): Mint address of Token A (deposited by maker).
    - `mint_b` (Pubkey): Mint address of Token B (requested by maker).
    - `bump` (u8): Bump seed for the escrow PDA.
    - `receive_amount` (u64): Amount of Token B the maker expects to receive.
  - **Purpose**: Stores metadata for the escrow and acts as the authority for the vault account.
  - **PDA Seeds**: `[b"escrow", maker.key(), seeds.to_le_bytes()]`.

### Instructions

#### 1. Make
Initializes an escrow and deposits Token A into a program-owned vault.
- **Accounts**:
  - `maker`: Signer who initiates the escrow and pays for account creation.
  - `mint_a`, `mint_b`: Mint addresses for Token A (deposited) and Token B (requested).
  - `maker_ata_for_token_a`: Maker's associated token account (ATA) holding Token A.
  - `escrow`: PDA (`[b"escrow", maker.key(), seeds.to_le_bytes()]`) to store escrow metadata.
  - `vault`: ATA owned by the `escrow` PDA, holding Token A.
  - `associated_token_program`, `token_program`, `system_program`: External programs for token and account operations.
- **Parameters**:
  - `discriminator` (u64): Unique seed for PDA derivation.
  - `receive_amount` (u64): Amount of Token B the maker expects.
  - `deposit_amount` (u64): Amount of Token A to deposit into the vault.
- **Behavior**:
  - Initializes the `escrow` account with metadata (seeds, maker, mints, bump, receive amount).
  - Transfers `deposit_amount` of Token A from `maker_ata_for_token_a` to the `vault` using `transfer_checked` to ensure correct decimals.

#### 2. Take
Completes the escrow by swapping Token B for Token A and closing accounts.
- **Accounts**:
  - `taker`, `maker`: Signers; taker completes the swap, maker receives Token B.
  - `mint_a`, `mint_b`: Mint addresses for the swap pair.
  - `taker_ata_for_token_a`: Taker's ATA to receive Token A (initialized if needed).
  - `taker_ata_for_token_b`: Taker's ATA holding Token B.
  - `maker_ata_for_token_b`: Maker's ATA to receive Token B (initialized if needed).
  - `vault`: ATA holding Token A, owned by `escrow`.
  - `escrow`: PDA to be closed, with rent refunded to the maker.
  - `associated_token_program`, `token_program`, `system_program`.
- **Behavior**:
  - Transfers `receive_amount` of Token B from `taker_ata_for_token_b` to `maker_ata_for_token_b`.
  - Transfers the full amount of Token A from `vault` to `taker_ata_for_token_a` using signer seeds for authorization.
  - Closes the `vault` and `escrow` accounts, refunding rent to the `maker`.

#### 3. Refund
Allows the maker to reclaim Token A and close the escrow if no taker completes the swap.
- **Accounts**:
  - `maker`: Signer who initiated the escrow.
  - `mint_a`: Mint address of Token A.
  - `maker_ata_for_token_a`: Maker's ATA to receive Token A.
  - `escrow`: PDA to be closed, with rent refunded to the maker.
  - `vault`: ATA holding Token A, to be closed.
  - `token_program`, `associated_token_program`.
- **Behavior**:
  - Transfers the full amount of Token A from `vault` to `maker_ata_for_token_a` using signer seeds.
  - Closes the `vault` and `escrow` accounts, refunding rent to the `maker`.

## Security Considerations
- **PDA Security**: The `escrow` PDA uses seeds `[b"escrow", maker.key(), seeds.to_le_bytes()]` with a bump to ensure uniqueness and prevent unauthorized access.
- **Vault Ownership**: The `vault` ATA is owned by the `escrow` PDA, ensuring only authorized instructions (`take` or `refund`) can transfer tokens.
- **Signer Seeds**: Used in `take` and `refund` to authorize token transfers and account closures, leveraging the `escrow` PDA's authority.
- **Account Constraints**: The `has_one` constraint ensures the `escrow` account is tied to the correct `maker`, `mint_a`, and `mint_b`. Seed validation prevents incorrect account usage.
- **Rent Reclamation**: Closing `escrow` and `vault` accounts in `take` and `refund` refunds rent to the maker, preventing state bloat.
- **Token Safety**: Uses `transfer_checked` to verify token mint and decimals, ensuring secure transfers.

## Testing
To test the program:
1. Start a local Solana validator:
   ```bash
   solana-test-validator
   ```
2. Run the test suite:
   ```bash
   anchor test
   ```
The test suite should cover:
- Creating an escrow with valid parameters and depositing tokens.
- Completing an escrow swap via `take`, verifying token transfers and account closures.
- Refunding an escrow via `refund`, ensuring tokens are returned and accounts are closed.
- Error cases, such as invalid seeds, unauthorized access, or incorrect token mints.

## Usage Example
1. **Create an Escrow**:
   - Call `make` with a unique `discriminator`, `receive_amount` (Token B to receive), and `deposit_amount` (Token A to deposit).
   - The maker deposits Token A into the `vault`, and the `escrow` PDA is initialized with swap details.
2. **Complete the Swap**:
   - A taker calls `take` to send the specified `receive_amount` of Token B to the maker's ATA.
   - The taker receives Token A from the `vault`, and both the `escrow` and `vault` accounts are closed, refunding rent to the maker.
3. **Refund the Escrow**:
   - If no taker completes the swap, the maker calls `refund` to reclaim Token A from the `vault`.
   - The `escrow` and `vault` accounts are closed, with rent refunded to the maker.

## Contributing
Contributions are welcome! Please:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

