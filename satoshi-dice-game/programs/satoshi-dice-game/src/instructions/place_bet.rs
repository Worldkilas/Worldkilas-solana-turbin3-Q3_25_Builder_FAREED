use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::state::Bet;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: This is safe
    pub house: UncheckedAccount<'info>,

    #[account(
        init,
        payer=player,
        space= 8+Bet::INIT_SPACE,
        seeds=[b"bet", vault.key().as_ref(), seed.to_be_bytes().as_ref()],
        bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds=[b"vault", house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        amount: u64,
        seed: u64,
        roll: u8,
        bumps: &PlaceBetBumps,
    ) -> Result<()> {
        self.bet.set_inner(Bet {
            player: self.player.key(),
            bet_amount: amount,
            seed,
            slot: Clock::get()?.slot,
            roll,
            bump: bumps.bet,
        });
        Ok(())
    }

    pub fn deposit_bet(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        transfer(cpi_ctx, amount)
    }
}
