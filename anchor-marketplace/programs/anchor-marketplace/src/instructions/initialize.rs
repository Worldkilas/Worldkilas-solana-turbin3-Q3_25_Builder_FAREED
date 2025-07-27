use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::marketplace::Marketplace;


/// Accounts required to initialize a new marketplace.
/// 
/// The initializer (admin) creates a unique marketplace with a reward mint and treasury,
/// identified by a name and derived PDA seeds.
/// 
/// # Seeds
/// - `marketplace`: `[b"marketplace", name.as_bytes()]`
/// - `treasury`: `[b"treasury", marketplace.key().as_ref()]`
/// - `rewards_mint`: `[b"rewards", marketplace.key().as_ref()]`
#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeMarketplace<'info> {
     /// The admin creating the marketplace.
    #[account(mut)]
    pub admin: Signer<'info>,

     /// PDA account representing the marketplace config.
    /// Initialized with a unique seed derived from the name.
    #[account(
        init,
        payer=admin,
        seeds=[b"marketplace", name.as_str().as_bytes()],
        bump,
        space= Marketplace::INIT_SPACE

    )]
    pub marketplace: Account<'info, Marketplace>,

      /// PDA for holding platform or protocol fees (in SOL).
    /// This is a system account controlled by the program.
    #[account(
        seeds=[b"treasury", marketplace.key().as_ref()],
        bump
    )]
    pub treasury: SystemAccount<'info>,


    /// SPL token mint for reward tokens tied to the marketplace.
    /// Mint authority is set to the marketplace PDA.
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
    /// Initializes the marketplace with admin authority, fee structure, and bumps.
    /// 
    /// # Arguments
    /// - `name`: Unique name for the marketplace, used in PDA seed.
    /// - `fee`: Fee percentage (basis points) taken on each sale (e.g., 250 = 2.5%).
    /// - `bumps`: Object holding all the PDA bumps for seeds used.
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
