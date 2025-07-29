pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HsRzQ5QEwLrtBVNTBbHQHJAAxvmqD9j2xwAHsNWZawUo");

#[program]
pub mod satoshi_dice_game {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64, seed: u64, roll: u8) -> Result<()> {
        ctx.accounts.create_bet(amount, seed, roll, &ctx.bumps)?;

        ctx.accounts.deposit_bet(amount)
    }

    pub fn refund_bet(ctx: Context<RefundBet>) -> Result<()> {
        ctx.accounts.refund_bet(&ctx.bumps)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: [u8;64]) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(&sig)?;
        ctx.accounts.resolve_bet(&sig, &ctx.bumps)
    }
}
