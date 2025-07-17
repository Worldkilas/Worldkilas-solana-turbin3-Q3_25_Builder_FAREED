// #![allow(unexpected_cfgs)]
// #![allow(deprecated)]
pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6JxFyXM7RVmXYE8HvEvFiddxy3E6q2feyhV7ihx7PH1k");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        discriminator: u64,
        receive_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        msg!("Initializing escrow");
        ctx.accounts
            .init_escrow(discriminator, receive_amount, &ctx.bumps)?;

        msg!(
            "Depositing {} tokens of {:?} into vault",
            deposit_amount,
            ctx.accounts.mint_a.key()
        );
        ctx.accounts.deposit_to_vault(deposit_amount)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        msg!("Depositing {:?} to maker", ctx.accounts.mint_b.key());
        ctx.accounts.deposit_to_maker()?;

        msg!("Widrawing {:?} from vault", ctx.accounts.mint_a.key());
        ctx.accounts.withdraw_from_vault_and_close()?;

        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        msg!(
            "Refunding {:?} back to maker account and closing the account",
            ctx.accounts.mint_a.key()
        );
        ctx.accounts.refund_and_close()?;
        Ok(())
    }
}
