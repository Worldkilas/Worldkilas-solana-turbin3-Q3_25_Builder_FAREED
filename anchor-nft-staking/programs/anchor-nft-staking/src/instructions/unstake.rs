use anchor_lang::prelude::*;
use anchor_spl::{metadata::{mpl_token_metadata:: instructions::{
    ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
}, MasterEditionAccount, Metadata, MetadataAccount},token::{revoke, Mint, Revoke, Token, TokenAccount}};

use crate::{StakeAccount, StakeConfig, StakerUserAccount,error::StakeError};



#[derive(Accounts)]
pub struct Unstake<'info>{
    #[account(mut)]
    pub staker: Signer<'info>,
    pub nft_mint: Account<'info, Mint>,
    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=staker
    )]
    pub staker_ata_mint: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"stake_config"],
        bump=stake_config.stake_config_bump
    )]
    pub stake_config: Account<'info,StakeConfig>,

    #[account(
        mut,
        close=staker,
        seeds=[b"stake", nft_mint.key().as_ref(),stake_config.key().as_ref()],
        bump= stake_account.stake_acct_bump
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds=[b"user_account", staker.key().as_ref()],
        bump= staker_account.staker_user_acct_bump
    )]
    pub staker_account: Account<'info, StakerUserAccount>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program=metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,

}

impl <'info> Unstake <'info> {
    pub fn unstake(&mut self)-> Result<()> {
        //calculate time elapsed since the user staked the NFT
        let time_elapsed=Clock::get()?.unix_timestamp-self.stake_account.staked_at;
        let time_elapsed=(time_elapsed/86400 ) as u32;

        require!(time_elapsed>=self.stake_config.freeze_period, StakeError::FreezePeriodNotElasped); 
        // icrease user points
        self.staker_account.points+=(self.stake_config.points_per_stake as u32)*time_elapsed;

        // thaw the nft accounts
        self.thaw_nft_account()?;

        //revoke the nft authority from stake_account
        self.revoke_nft_authority()?;

        self.staker_account.amount_staked-=1;

        Ok(())
    }

    pub fn thaw_nft_account(&mut self)->Result<()> {
        let cpi_program=self.metadata_program.to_account_info();

        let signers_seeds=&[
            b"stake",
            self.nft_mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_account.stake_acct_bump]
        ];

        let signers_seeds=&[&signers_seeds[..]];

        let thaw_accounts= ThawDelegatedAccountCpiAccounts{
            delegate: &self.stake_account.to_account_info(),
            edition: &self.master_edition.to_account_info(),
            token_account: &self.staker_ata_mint.to_account_info(),
            mint: &self.nft_mint.to_account_info(),
            token_program: &self.token_program.to_account_info()
        };

        ThawDelegatedAccountCpi::new(&cpi_program, thaw_accounts).invoke_signed(signers_seeds)?;

        Ok(())
    }

    pub fn revoke_nft_authority(&mut self)-> Result<()> {
        let cpi_program= self.token_program.to_account_info();
        let revoke_accounts=Revoke{
            source: self.staker_ata_mint.to_account_info(),
            authority: self.staker.to_account_info()
        };

        let revoke_ctx= CpiContext::new(cpi_program, revoke_accounts);
        revoke(revoke_ctx)?;
        Ok(())
    }


}