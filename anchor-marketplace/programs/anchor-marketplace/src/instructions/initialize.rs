use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::marketplace::Marketplace;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer=admin,
        seeds=[b"marketplace", name.as_str().as_bytes()],
        bump,
        space= Marketplace::INIT_SPACE

    )]
    pub marketplace: Account<'info, Marketplace>,

    #[account(
        seeds=[b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer=admin,
        seeds=[b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=marketplace
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeMarketplace<'info> {
    pub fn init_marketplace(
        &mut self,
        name: String,
        fee: u16,
        bumps: &InitializeMarketplaceBumps,
    ) -> Result<()> {
        self.marketplace.set_inner(Marketplace {
            admin: self.admin.key(),
            fee,
            name,
            treasury_bump: bumps.treasury,
            rewards_bump: bumps.rewards_mint,
            marketplace_bump: bumps.marketplace,
        });
        Ok(())
    }
}
