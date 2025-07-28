use anchor_lang::{
    accounts::signer,
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::Bet;

#[derive(Accounts)]
pub struct RefundBet<'info> {
    #[account(mut)]
    pub player: Signer<'info>,

    /// CHECK: This is only used for signing purpose
    pub house: UncheckedAccount<'info>,

    #[account(
        mut,
        close=player,
        has_one=player,
        seeds=[b"bet", vault.key().as_ref(), bet.seed.to_be_bytes().as_ref()],
        bump=bet.bump,
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        mut,
        seeds=[b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> RefundBet<'info> {
    pub fn refund_bet(&mut self, bumps: &RefundBetBumps) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };

        let signer_seeds = &[
            b"vault",
            self.house.to_account_info().key.as_ref(),
            &[bumps.vault],
        ];

        let signer_seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi_ctx, self.bet.bet_amount)
    }
}
