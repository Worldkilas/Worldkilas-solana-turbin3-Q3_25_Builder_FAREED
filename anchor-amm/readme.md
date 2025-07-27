# Anchor AMM (Automated Market Maker)

This repository contains a Solana program built with **Anchor** for an Automated Market Maker (AMM) that facilitates decentralized token swaps, liquidity provision, and withdrawals using a constant product curve. The program allows users to initialize liquidity pools, deposit tokens to provide liquidity, swap tokens, and withdraw liquidity while ensuring secure token transfers and slippage protection.

## Overview

The **Anchor AMM** is designed to support:

- **Initialization** of liquidity pools with configurable fees and token pairs.
- **Depositing** liquidity to a pool in exchange for LP (liquidity provider) tokens.
- **Swapping** tokens (X to Y or Y to X) with a constant product formula and fee application.
- **Withdrawing** liquidity by burning LP tokens to reclaim tokens from the pool.
- **Security Features** like slippage protection, pool locking, and PDA-based account management.

The program uses the **SPL Token Program** for token operations and implements a constant product curve (`constant_product_curve::ConstantProduct`) for pricing. Program Derived Addresses (PDAs) ensure secure ownership of pool vaults and LP mints.

## Program ID

The program ID is: `6HPwmQCHtUC5kgwo4KvMRAWisuZJi9XZwEJCn7q3vkte`

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
   cd anchor-amm
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


### Data Structures

- **Config**: Stores pool metadata:
  - `seed`: Unique seed for PDA derivation.
  - `authority`: Optional admin override for pool control.
  - `token_x_mint` and `token_y_mint`: Mint addresses for the token pair.
  - `fee`: Swap fee in basis points (e.g., 30 = 0.3%).
  - `locked`: Boolean to lock/unlock the pool.
  - `config_bump` and `lp_bump`: PDA bumps for `config` and `lp_token_mint`.

### Instructions

#### 1. Initialize

Creates a new liquidity pool with a token pair, fee structure, and LP token mint.

- **Accounts**:
  - `admin`: Signer who pays for account creation.
  - `token_x_mint` and `token_y_mint`: Mints for the token pair (e.g., USDC/SOL).
  - `lp_token_mint`: PDA for LP tokens (`seeds=[b"lp", config.key()]`).
  - `config`: PDA for pool metadata (`seeds=[b"config", seed.to_le_bytes()]`).
  - `pool_token_x_vault` and `pool_token_y_vault`: Associated token accounts (ATAs) owned by `config`.
- **Parameters** (via `InitArgs`):
  - `seed`: Unique seed for PDA derivation.
  - `fee`: Fee in basis points (e.g., 30 = 0.3%).
  - `authority`: Optional admin override (defaults to `admin` if `None`).
- **Behavior**:
  - Initializes the `config` account with pool metadata.
  - Creates the LP token mint with 6 decimals, controlled by `config`.
  - Sets up token vaults for `token_x` and `token_y`.

#### 2. Deposit

Allows users to add liquidity to the pool in exchange for LP tokens.

- **Accounts**:
  - `depositor`: Signer providing liquidity.
  - `token_x_mint` and `token_y_mint`: Token pair mints.
  - `config`: Pool configuration PDA.
  - `pool_token_x_vault` and `pool_token_y_vault`: Pool vaults for token storage.
  - `lp_token_mint`: LP token mint for issuing tokens.
  - `depositor_token_x_account` and `depositor_token_y_account`: Depositor’s ATAs.
  - `depositor_lp_token_account`: Depositor’s ATA for receiving LP tokens.
- **Parameters**:
  - `lp_amount_to_be_minted`: Desired amount of LP tokens to mint.
  - `max_x` and `max_y`: Maximum amounts of token X and Y to deposit (slippage protection).
- **Behavior**:
  - Checks if the pool is locked (`require_not_locked!`).
  - For an empty pool, uses `max_x` and `max_y` directly.
  - For a non-empty pool, calculates required `x` and `y` amounts using the constant product curve.
  - Ensures deposited amounts respect slippage limits (`x <= max_x`, `y <= max_y`).
  - Transfers tokens from depositor to pool vaults.
  - Mints LP tokens to `depositor_lp_token_account`.

#### 3. Swap

Facilitates token swaps (X to Y or Y to X) with a constant product curve and fees.

- **Accounts**:
  - `token_pair_swapper`: Signer performing the swap.
  - `token_x_mint` and `token_y_mint`: Token pair mints.
  - `config`: Pool configuration PDA.
  - `lp_token_mint`: LP token mint (for supply reference).
  - `pool_token_x_vault` and `pool_token_y_vault`: Pool vaults.
  - `swapper_token_x_account` and `swapper_token_y_account`: Swapper’s ATAs.
- **Parameters**:
  - `is_token_x`: `true` for X-to-Y swap, `false` for Y-to-X.
  - `amount_in`: Input token amount.
  - `min_amount_out`: Minimum output token amount (slippage protection).
- **Behavior**:
  - Checks if the pool is locked and ensures `amount_in > 0`.
  - Uses the constant product curve to calculate swap amounts, applying fees.
  - Ensures output respects `min_amount_out` to prevent excessive slippage.
  - Transfers input tokens from swapper to the appropriate vault.
  - Transfers output tokens from the other vault to the swapper.

#### 4. Withdraw

Allows liquidity providers to burn LP tokens and reclaim tokens from the pool.

- **Accounts**:
  - `withdrawer`: Signer withdrawing liquidity.
  - `token_x_mint` and `token_y_mint`: Token pair mints.
  - `config`: Pool configuration PDA.
  - `lp_token_mint`: LP token mint.
  - `pool_token_x_vault` and `pool_token_y_vault`: Pool vaults.
  - `withdrawer_token_x_account` and `withdrawer_token_y_account`: Withdrawer’s ATAs.
  - `withdrawer_lp_token_account`: Withdrawer’s ATA holding LP tokens.
- **Parameters**:
  - `lp_amount_to_be_burned`: Amount of LP tokens to burn.
  - `min_x` and `min_y`: Minimum amounts of token X and Y to withdraw (slippage protection).
- **Behavior**:
  - Checks if the pool is locked and ensures non-zero LP amount.
  - Verifies pool has liquidity (`NoLiquidityInPool` error if empty).
  - Calculates withdrawal amounts using the constant product curve.
  - Ensures withdrawn amounts meet `min_x` and `min_y` (slippage protection).
  - Transfers tokens from vaults to withdrawer’s ATAs.
  - Burns the specified LP tokens.

## Error Handling

The `AmmError` enum defines custom errors, including:

- `DefaultError`, `OfferExpired`, `PoolLocked`, `SlippageExceeded`, `Overflow`, `Underflow`.
- `InvalidToken`, `LiquidityLessThanMinimum`, `NoLiquidityInPool`, `BumpError`, `CurveError`.
- `InvalidFee`, `InvalidAuthority`, `NoAuthoritySet`, `InvalidAmount`, `InvalidPrecision`.
- `InsufficientBalance`, `ZeroBalance`.
  Errors from the constant product curve are mapped to `AmmError` for consistent handling.

## Security Considerations

- **PDA Security**: Uses seeds (`config`: `[b"config", seed.to_le_bytes()]`, `lp_token_mint`: `[b"lp", config.key()]`) and bumps to ensure unique, secure PDAs.
- **Pool Locking**: The `locked` flag prevents operations when the pool is locked, controlled by the `authority`.
- **Slippage Protection**: `max_x`, `max_y` (deposit) and `min_x`, `min_y` (withdraw), `min_amount_out` (swap) prevent unfavorable trades.
- **Non-Zero Checks**: `require_non_zero!` macro ensures no zero-amount operations.
- **Token Vault Ownership**: Vaults are owned by the `config` PDA, ensuring only authorized instructions can transfer tokens.
- **Constant Product Curve**: Ensures fair pricing and prevents pool depletion through mathematical constraints.

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

- Pool initialization with valid/invalid parameters.
- Depositing liquidity (initial and subsequent deposits).
- Swapping tokens in both directions (X-to-Y, Y-to-X).
- Withdrawing liquidity with slippage checks.
- Error cases (e.g., locked pool, insufficient balance, slippage exceeded).

## Usage Example

1. **Initialize a Pool**:
   Call `initialize` with a unique `seed`, fee (e.g., 30 = 0.3%), and optional `authority`. This sets up the `config`, `lp_token_mint`, and vaults.
2. **Deposit Liquidity**:
   Use `deposit` to add tokens to the pool, specifying `lp_amount_to_be_minted`, `max_x`, and `max_y`. Receive LP tokens in return.
3. **Swap Tokens**:
   Call `swap` with `is_token_x` (direction), `amount_in`, and `min_amount_out` to exchange tokens.
4. **Withdraw Liquidity**:
   Use `withdraw` to burn LP tokens (`lp_amount_to_be_burned`) and reclaim tokens, specifying `min_x` and `min_y` for slippage protection.

## Explanation of Key Components

- **Constant Product Curve**: The `constant_product_curve::ConstantProduct` library enforces the `x * y = k` formula for pricing, ensuring pool balance. It handles calculations for swaps, deposits, and withdrawals, applying fees and checking slippage.
- **PDA Management**: The `config` and `lp_token_mint` PDAs ensure secure ownership and control. The `config` PDA owns vaults and the LP mint, while bumps prevent seed collisions.
- **Token Transfers**: Use `transfer_checked` for secure SPL token transfers, respecting token decimals and authority checks.
- **Slippage Protection**: Parameters like `min_amount_out`, `max_x`, `max_y`, `min_x`, and `min_y` protect users from unfavorable price changes.
- **Error Mapping**: Errors from the curve library are converted to `AmmError` for consistent user feedback.
- **Macros**: `require_non_zero!` and `require_not_locked!` simplify validation checks across instructions.

## Contributing

Contributions are welcome! Please:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit changes (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
