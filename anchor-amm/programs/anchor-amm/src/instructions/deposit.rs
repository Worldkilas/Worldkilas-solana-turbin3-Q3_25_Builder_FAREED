use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, require_non_zero, require_not_locked, state::Config};

#[derive(Accounts)]
pub struct Deposit<'info> {
    /// The user (signer) who is adding liquidity to the AMM pool.
    #[account(mut)]
    pub depositor: Signer<'info>,
    /// The mint address of Token X (e.g., USDC).
    #[account(mint::token_program=token_program)]
    pub token_x_mint: Account<'info, Mint>,
    /// The mint address of Token Y (e.g., SOL)
    #[account(mint::token_program=token_program)]
    pub token_y_mint: Account<'info, Mint>,

    /// Global AMM configuration PDA that holds metadata like fees and bumps.
    /// - PDA derived from seed `[b"config", config.seed.to_le_bytes()]`
    #[account(
        has_one=token_x_mint,
        has_one=token_y_mint,
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump= config.config_bump
     )]
    pub config: Account<'info, Config>,

    /// The vault PDA account that stores Token X liquidity inside the pool.
    /// - Owned by the AMM (authority = config)
    #[account(
        mut,
        associated_token::mint=token_x_mint,
        associated_token::authority=config
    )]
    pub pool_token_x_vault: Account<'info, TokenAccount>,

    /// The vault PDA account that stores Token Y liquidity inside the pool.
    /// - Owned by the AMM (authority = config)
    #[account(
        mut,
        associated_token::mint=token_y_mint,
        associated_token::authority=config
    )]
    pub pool_token_y_vault: Account<'info, TokenAccount>,

    /// needs to be mut to change total supply of mint as
    /// Every time a user deposits a trading pair. Addition of users causes the total
    /// supply to increase and vice-versa
    #[account(
        mut,
        seeds= [b"lp",config.key().as_ref()],
        bump= config.lp_bump,
    )]
    pub lp_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=token_x_mint,
        associated_token::authority=config
    )]
    /// The depositor’s associated token account for Token X.
    /// - Tokens will be debited from here and transferred into `pool_token_x_vault`.
    pub depositor_token_x_account: Account<'info, TokenAccount>,
    /// The depositor’s associated token account for Token Y.
    /// - Tokens will be debited from here and transferred into `pool_token_y_vault`.
    #[account(
        mut,
        associated_token::mint=token_y_mint,
        associated_token::authority=config
    )]
    pub depositor_token_y_account: Account<'info, TokenAccount>,

    /// The depositor’s associated token account for the LP token.
    /// - Will receive newly minted LP tokens in exchange for providing liquidity.
    /// - `init_if_needed` ensures this account is created if it doesn't exist.
    #[account(
        init_if_needed,
        payer= depositor,
        associated_token::mint=lp_token_mint,
        associated_token::authority=depositor

    )]
    pub depositor_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, lp_amount_to_be_minted: u64, max_x: u64, max_y: u64) -> Result<()> {
        require_not_locked!(self.config.locked);
        require_non_zero!([lp_amount_to_be_minted, max_x, max_y]);
        let (x, y) = match self.lp_token_mint.supply == 0
            && self.pool_token_x_vault.amount == 0
            && self.pool_token_y_vault.amount == 0
        {
            true => (max_x, max_y),
            false => {
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.pool_token_x_vault.amount,
                    self.pool_token_y_vault.amount,
                    self.lp_token_mint.supply,
                    lp_amount_to_be_minted,
                    6,
                )
                .map_err(AmmError::from)?;
                (amount.x, amount.y)
            }
        };
        require!(x <= max_x && y <= max_y, AmmError::SlippageExceeded);
        // deposit x and y tokens
        self.deposit_tokens(true, x)?;
        self.deposit_tokens(false, y)?;

        self.mint_lp_tokens(lp_amount_to_be_minted)?;
        Ok(())
    }
    pub fn deposit_tokens(&mut self, is_token_x: bool, amount: u64) -> Result<()> {
        let (from, to) = match is_token_x {
            true => (
                self.depositor_token_x_account.to_account_info(),
                self.pool_token_x_vault.to_account_info(),
            ),
            false => (
                self.depositor_token_y_account.to_account_info(),
                self.pool_token_y_vault.to_account_info(),
            ),
        };
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer {
            from,
            to,
            authority: self.depositor.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;
        Ok(())
    }

    pub fn mint_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = MintTo {
            mint: self.lp_token_mint.to_account_info(),
            to: self.depositor_lp_token_account.to_account_info(),
            authority: self.config.to_account_info(),
        };
        let signer_seeds = &[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(cpi_ctx, amount)?;
        Ok(())
    }
}
