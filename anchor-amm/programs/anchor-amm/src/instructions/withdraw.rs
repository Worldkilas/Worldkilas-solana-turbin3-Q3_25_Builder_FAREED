use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
    token::{burn, transfer_checked, Burn, Mint, Token, TokenAccount, TransferChecked},
};
use constant_product_curve::ConstantProduct;

use crate::{error::AmmError, require_non_zero, require_not_locked, state, Config};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub withdrawer: Signer<'info>,
    #[account(mint::token_program=token_program)]
    pub token_x_mint: Account<'info, Mint>,
    #[account(mint::token_program=token_program)]
    pub token_y_mint: Account<'info, Mint>,

    #[account(
        has_one=token_x_mint,
        has_one=token_y_mint,
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump=config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump= config.lp_bump
    )]
    pub lp_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=token_x_mint,
        associated_token::authority=config     
    )]
    pub pool_token_x_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_y_mint,
        associated_token::authority=config     
    )]
    pub pool_token_y_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_y_mint,
        associated_token::authority=config
    )]
    pub withdrawer_token_x_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_y_mint,
        associated_token::authority=config
    )]
    pub withdrawer_token_y_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=lp_token_mint,
        associated_token::authority=withdrawer

    )]
    pub withdrawer_lp_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, lp_amount_to_be_burned: u64, min_x: u64, min_y: u64) -> Result<()> {
        require_not_locked!(self.config.locked);
        require_non_zero!([lp_amount_to_be_burned]);
        require!(
            !(self.lp_token_mint.supply==0&& self.pool_token_x_vault.amount==0&& self.pool_token_y_vault.amount==0),
            AmmError::NoLiquidityInPool,
        );


        let (x, y) = {
            let withdraw_amount = ConstantProduct::xy_withdraw_amounts_from_l(
                self.pool_token_x_vault.amount,
                self.pool_token_y_vault.amount,
                self.lp_token_mint.supply,
                lp_amount_to_be_burned,
                6,
            )
            .map_err(AmmError::from)?;
            (withdraw_amount.x, withdraw_amount.y)
        };
        require!(x>= min_x&& y>=min_y, AmmError::SlippageExceeded);
        self.withdraw_tokens(x, true)?;
        self.withdraw_tokens(y, false)?;
        self.burn_lp_tokens(lp_amount_to_be_burned)?;

        Ok(())
    }

    pub fn withdraw_tokens(&mut self, amount: u64, is_token_x: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_token_x {
            true => (
                self.pool_token_x_vault.to_account_info(),
                self.withdrawer_token_x_account.to_account_info(),
                self.token_x_mint.to_account_info(),
                self.token_x_mint.decimals,
            ),
            false => (
                self.pool_token_y_vault.to_account_info(),
                self.withdrawer_token_y_account.to_account_info(),
                self.token_y_mint.to_account_info(),
                self.token_y_mint.decimals,
            ),
        };
        let cpi_program = self.token_program.to_account_info();
        let transfer_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.config.to_account_info(),
        };
        let signer_seeds = &[
            b"config",
            &self.config.seed.to_le_bytes()[..],
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);
        transfer_checked(cpi_ctx, amount, decimals)?;
        Ok(())
    }

    pub fn burn_lp_tokens(&mut self, amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let burn_accounts = Burn {
            mint: self.lp_token_mint.to_account_info(),
            from: self.withdrawer_lp_token_account.to_account_info(),
            authority: self.withdrawer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, burn_accounts);
        burn(cpi_ctx, amount)?;
        Ok(())
    }
}
