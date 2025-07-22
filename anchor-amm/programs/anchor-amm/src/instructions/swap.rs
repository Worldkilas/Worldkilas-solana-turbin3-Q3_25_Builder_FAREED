use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{error::AmmError, require_not_locked, Config};

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub token_pair_swapper: Signer<'info>,

    #[account(mint::token_program=token_program)]
    pub token_x_mint: Account<'info, Mint>,

    #[account(mint::token_program=token_program)]
    pub token_y_mint: Account<'info, Mint>,

    #[account(
        has_one=token_x_mint,
        has_one=token_y_mint,
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump= config.config_bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds=[b"lp", config.key().as_ref()],
        bump=config.lp_bump

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
        init_if_needed,
        payer=token_pair_swapper,
        associated_token::mint= token_x_mint,
        associated_token::authority=token_pair_swapper,
        associated_token::token_program=token_program

    )]
    pub swapper_token_x_account: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer= token_pair_swapper,
        associated_token::mint=token_y_mint,
        associated_token::authority=token_pair_swapper,
        associated_token::token_program=token_program
    )]
    pub swapper_token_y_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    /// Performs a token swap from either token X to Y or Y to X.
    ///
    /// - `is_token_x`: If true, user is swapping token X for token Y.
    /// - `amount_in`: The amount of the input token to swap.
    /// - `min_amount_out`: The minimum acceptable amount of the output token (slippage protection).
    pub fn swap_tokens(
        &mut self,
        is_token_x: bool,
        amount_in: u64,
        min_amount_out: u64,
    ) -> Result<()> {
        require_not_locked!(self.config.locked);
        require!(amount_in > 0, AmmError::InvalidAmount);

         // Initialize the curve with current pool state: balances, supply, fee, etc.
        let mut curve = ConstantProduct::init(
            self.pool_token_x_vault.amount,
            self.pool_token_y_vault.amount,
            self.lp_token_mint.supply,
            self.config.fee,
            None,
        )
        .map_err(AmmError::from)?;

    // Determine swap direction: X to Y or Y to X.
        let pair = if is_token_x {
            LiquidityPair::X
        } else {
            LiquidityPair::Y
        };

        // Perform the swap on the curve, passing direction, input, and slippage constraint.
        let swap_results = curve
            .swap(pair, amount_in, min_amount_out)
            .map_err(AmmError::from)?;

        require!(swap_results.deposit!=0, AmmError::InvalidAmount);
        require!(swap_results.deposit!=0, AmmError::InvalidAmount);

        self.deposit_tokens(is_token_x, swap_results.deposit)?;
        self.withdraw_tokens(!is_token_x, swap_results.withdraw)?;


        Ok(())
    }


    /// Transfers the input token from the swapper to the pool vault.
    ///
    /// - `is_token_x`: true if input token is token X; false if it's token Y.
    /// - `deposit_amount`: the amount of tokens to deposit.
    pub fn deposit_tokens(&mut self, is_token_x: bool, deposit_amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_token_x {
            true => (
                self.token_pair_swapper.to_account_info(),
                self.pool_token_x_vault.to_account_info(),
                self.token_x_mint.to_account_info(),
                self.token_x_mint.decimals,
            ),
            false => (
                self.token_pair_swapper.to_account_info(),
                self.pool_token_y_vault.to_account_info(),
                self.token_y_mint.to_account_info(),
                self.token_y_mint.decimals,
            ),
        };
        let cpi_program = self.token_program.to_account_info();

        let transfer_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.token_pair_swapper.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, transfer_accounts);

        transfer_checked(cpi_ctx, deposit_amount, decimals)?;
        Ok(())
    }

    /// Transfers the output token from the pool vault to the swapper.
    ///
    /// - `is_token_x`: true if output token is token X; false if it's token Y.
    /// - `withdraw_amount`: the amount of tokens to withdraw to the user.
    pub fn withdraw_tokens(&mut self, is_token_x: bool,withdraw_amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_token_x {
            true => (
                self.pool_token_x_vault.to_account_info(),
                self.token_pair_swapper.to_account_info(),
                self.token_x_mint.to_account_info(),
                self.token_x_mint.decimals,
            ),
            false => (
                self.pool_token_y_vault.to_account_info(),
                self.token_pair_swapper.to_account_info(),
                self.token_y_mint.to_account_info(),
                self.token_y_mint.decimals,
            ),
        };
        let cpi_program = self.token_program.to_account_info();
        let withdraw_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.config.to_account_info(),
        };
        let signer_seeds = &[
            b"config",
            &self.config.seed.to_be_bytes()[..],
            &[self.config.config_bump],
        ];
        let signer_seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, withdraw_accounts, signer_seeds);

        transfer_checked(cpi_ctx, withdraw_amount, decimals)?;
        Ok(())
    }
}
