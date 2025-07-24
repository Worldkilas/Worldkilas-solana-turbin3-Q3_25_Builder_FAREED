use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::Config;
/// Accounts required for initializing the liquidity pool.
///
/// This instruction performs the following actions:
/// - Creates and initializes the config account using the provided `seed`.
/// - Creates a new LP token mint controlled by the config.
/// - Initializes token vaults (associated token accounts) for token X and token Y, 
///   owned by the config.
/// - Allocates space for the config account and seeds the LP mint and config PDAs.
///
/// The `admin` pays for all account creations.
/// 
/// ## PDA Seeds
/// - `config` PDA: `["config", seed.to_le_bytes()]`
/// - `lp_token_mint` PDA: `["lp", config.key()]`
///
/// ## Constraints
/// - `token_x_mint` and `token_y_mint` must be valid SPL token mints.
/// - The LP mint is created with 6 decimal places and authority set to the config.
/// - The token vaults are created as associated token accounts with the config as the owner.

#[derive(Accounts)]
#[instruction(seed:u64 )]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    pub token_x_mint: Account<'info, Mint>,
    pub token_y_mint: Account<'info, Mint>,
    #[account(
        init,
        payer=admin,
        seeds=[b"lp",config.key().as_ref()],
        bump,
        mint::decimals=6,
        mint::authority=config

    )]
    pub lp_token_mint: Account<'info, Mint>,
    #[account(
        init,
        payer=admin,
        seeds=[b"config", seed.to_le_bytes().as_ref()],
        bump,
        space= 8+Config::INIT_SPACE
    )]
    pub config: Account<'info, Config>,
    #[account(
        init,
        payer= admin,
        associated_token::mint=token_x_mint,
        associated_token::authority=config,
    )]
    pub pool_token_x_vault: Account<'info, TokenAccount>,
    #[account(
        init, 
        payer= admin,
        associated_token::mint=token_y_mint,
        associated_token::authority=config
    )]
    pub pool_token_y_vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.config.set_inner(Config {
            seed,
            authority,
            token_x_mint: self.token_x_mint.key(),
            token_y_mint: self.token_y_mint.key(),
            fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.lp_token_mint,
        });
        Ok(())
    }
}
