use anchor_lang::{
    prelude::*,
    solana_program::sysvar::rent,
    system_program::{transfer, Transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{MarketplaceConfig, CONFIG_BINARY_STRING, TREASURY_BINARY_STRING};


#[derive(Accounts)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer= authority,
        space= 8+ MarketplaceConfig::INIT_SPACE,
        seeds=[CONFIG_BINARY_STRING, authority.key().as_ref()],
        bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        mut,
        seeds=[TREASURY_BINARY_STRING,marketplace_config.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer= authority,
        associated_token::mint=token_mint,
        associated_token::authority=treasury
    )]
    pub treasury_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn init_marketplace(
        &mut self,
        commit_fees_bps: u16,
        withdraw_fees_bps: u16,
        bumps: &InitializeMarketplaceBumps,
    ) -> Result<()> {
        self.marketplace_config.set_inner(MarketplaceConfig {
            authority: self.authority.key(),
            commit_fees_bps,
            withdraw_fees_bps,
            treasury_bump: bumps.treasury,
            bump: bumps.marketplace_config,
        });
        let amount_for_rent =
            rent::Rent::get()?.minimum_balance(self.treasury.to_account_info().data_len());

        let transfer_account = Transfer {
            from: self.authority.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_account);

        transfer(cpi_ctx, amount_for_rent)?;
        Ok(())
    }
}
