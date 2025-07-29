# Satoshi Dice Game

A provably fair dice game on Solana, built with Anchor. Players bet SOL, pick a roll threshold (2–96), and the house resolves the outcome using an Ed25519 signature for verifiable randomness. Win big if your roll beats the dice—or lose it all. The house takes a 1.5% edge, but the odds are in your hands.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Testing](#testing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Installation

To get this dice game running on Solana, you’ll need to set up your environment and deploy the program. Here’s how:

1. **Prerequisites**
   - [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.
   - [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools) (v1.18+ recommended).
   - [Anchor CLI](https://www.anchor-lang.com/docs/installation) (`anchor-cli`).
   - A Solana wallet with some SOL for deployment (e.g., via `solana-keygen`).

2. **Clone the Repo**
   ```bash
   git clone satoshi-dice-game
   cd satoshi-dice-game
   ```

3. **Install Dependencies**
   ```bash
   npm install  # If using a JS/TS client or tests
   anchor build
   ```

4. **Update Program ID**
   - Replace the `declare_id!("HsRzQ5QEwLrtBVNTBbHQHJAAxvmqD9j2xwAHsNWZawUo")` in `lib.rs` with your own program ID:
     ```bash
     solana-keygen new -o id.json
     anchor build
     ```
   - Update `Anchor.toml` with the new program ID.

5. **Deploy to Solana**
   - Set your network (e.g., devnet):
     ```bash
     solana config set --url https://api.devnet.solana.com
     ```
   - Deploy:
     ```bash
     anchor deploy
     ```s

6. **Fund the Vault**
   - The house needs to initialize the vault with SOL:
     ```bash
     anchor run initialize --amount <lamports>
     ```

You’re ready to roll! The program’s live on Solana, and the vault’s funded.

## Usage

Here’s how to interact with the Satoshi Dice Game—whether you’re the house or a player.

### For Players
1. **Place a Bet**
   - Pick an amount (in lamports, min 0.01 SOL), a seed (unique number), and a roll (2–96).
   - Call the `place_bet` instruction:
     ```rust
     place_bet(ctx: Context<PlaceBet>, amount: u64, seed: u64, roll: u8)
     ```
   - Example (via a client):
     ```javascript
     await program.methods.placeBet(amount, seed, roll).rpc();
     ```
   - This locks your SOL in the vault and creates a bet account.

2. **Wait for Resolution**
   - The house resolves your bet with a signature (see below).
   - If your `roll` beats the dice (1–100), you win a payout based on the odds.

3. **Refund (Optional)**
   - If the house doesn’t resolve in time, reclaim your bet:
     ```rust
     refund_bet(ctx: Context<RefundBet>)
     ```

### For the House
1. **Initialize the Vault**
   - Fund the vault to cover payouts:
     ```rust
     initialize(ctx: Context<Initialize>, amount: u64)
     ```

2. **Resolve Bets**
   - Generate an Ed25519 signature using the bet’s state (player, amount, seed, etc.).
   - Submit it to resolve:
     ```rust
     resolve_bet(ctx: Context<ResolveBet>, sig: [u8; 64])
     ```
   - The program hashes the signature for randomness, rolls a dice (1–100), and pays out if the player wins.

### How Payouts Work
- **Formula**: `payout = (bet_amount * (100% - 1.5%)) / ((roll - 1) * 100)` (in basis points).
- **Example**:
  - Bet: 10,000 lamports, Roll: 50.
  - Payout if you win: `(10,000 * 9,850) / (49 * 100) ≈ 20,102 lamports`.
- The higher your `roll`, the lower the payout—but the better your odds.

## Contributing

Want to improve the game? We’re open to contributions! Here’s how to get involved:

1. **Fork the Repo** and clone it locally.
2. **Submit Issues** for bugs or ideas—use GitHub Issues.
3. **Make Changes**:
   - Add features (e.g., UI, better randomness).
   - Fix bugs (check the `DiceError` enum for edge cases).
4. **Pull Requests**:
   - Keep code clean and follow Anchor conventions.
   - Test your changes (see below).
   - Submit a PR with a clear description.

Not accepting contributions? Let us know in the PR comments!

## Testing

For a software project like this, testing ensures the dice don’t lie. Here’s how to run tests:

1. **Write Tests**
   - Add them in `tests/` (e.g., `satoshi_dice_game.js`).
   - Example:
     ```javascript
     it("Places and resolves a bet", async () => {
       await program.methods.placeBet(...).rpc();
       // Test resolution logic
     });
     ```

2. **Run Tests**
   ```bash
   anchor test
   ```
   - This spins up a local Solana validator and runs your tests.

3. **Verify**
   - Check payout math, signature verification, and edge cases (e.g., min/max bets).

No tests yet? Start with the basics above and build from there.

## License

This project is released under the [MIT License](https://opensource.org/licenses/MIT). Feel free to use, modify, or distribute it—just keep the license notice intact.

## Acknowledgments

- **Anchor**: For making Solana dev a breeze.
- **Solana**: The blockchain powering this game.
- **Satoshi**: Inspiring provably fair gambling, wherever you are.