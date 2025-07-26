use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, Mint, MintTo, Token, TokenAccount}};

use crate::state::{StakerUserAccount,StakeConfig};

#[derive(Accounts)]
pub struct Claim<'info>{
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds=[b"user_account", staker.key().as_ref()],
        bump=staker_account.staker_user_acct_bump,
    )]
    pub staker_account: Account<'info, StakerUserAccount>,

    #[account(
        seeds=[b"stake_config"],
        bump=stake_config.stake_config_bump
    )]
    pub stake_config: Account<'info, StakeConfig>,

     /// The mint account from which reward tokens are minted.
    #[account(
        mut,
        seeds=[b"rewards", stake_config.key().as_ref()],
        bump= stake_config.rewards_bump
    )]
    pub rewards_mint: Account<'info, Mint>,

     /// Associated Token Account (ATA) of the staker to receive the reward tokens
    #[account(
        init_if_needed,
        payer=staker,
        associated_token::mint=rewards_mint,
        associated_token::authority=staker,
    )]
    pub staker_rewards_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info,AssociatedToken>

}

impl <'info> Claim<'info> {
     /// Mints staking reward tokens to the user's associated token account
    /// based on the number of points they have accumulated.
    pub fn claim(&mut self)->Result<()> {
        let cpi_program= self.token_program.to_account_info();
        let signer_seeds=&[
            b"stake_config".as_ref(),
            &[self.stake_config.stake_config_bump]
        ];

        let signer_seeds=&[&signer_seeds[..]];

        let mint_accounts= MintTo{
            mint: self.rewards_mint.to_account_info(),
            to: self.staker_rewards_ata.to_account_info(),
            authority: self.stake_config.to_account_info()
        };

        let cpi_ctx= CpiContext::new_with_signer(cpi_program, mint_accounts, signer_seeds);

         // Calculate reward token amount: points × 10^decimals
        let amount_to_mint=self.staker_account.points as u64 * 10_u64.pow(self.rewards_mint.decimals as u32);

        mint_to(cpi_ctx, amount_to_mint)?;
        self.staker_account.points=0;
        Ok(())
    }
}
