use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

use crate::{error::StakeError, StakeAccount, StakeConfig, StakerUserAccount};

/// Accounts context for the `stake` instruction.
///
/// This instruction allows a user to stake an NFT into the protocol by:
/// - Verifying the NFT belongs to a valid verified collection.
/// - Approving the stake account as a delegate of the NFT.
/// - Freezing the NFT so it cannot be transferred during staking.
/// - Recording staking metadata into a new `StakeAccount`.
///
/// The stake account is unique to a combination of:
/// - The NFT's mint address
/// - The global stake configuration
///
/// The staker’s stake data is tracked in their `UserAccount`.

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    /// The NFT being staked.
    pub nft_mint: Account<'info, Mint>,

    /// The mint of the verified collection this NFT must belong to.
    pub collection_mint: Account<'info, Mint>,

    /// The staker’s token account holding the NFT to be staked.
    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=staker,
    )]
    pub staker_mint_ata: Account<'info, TokenAccount>,

    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),nft_mint.key().as_ref()],
        seeds::program=metadata_program.key(),
        bump,
        constraint= metadata.collection.as_ref().unwrap().key.as_ref()==collection_mint.key().as_ref(),
        constraint=metadata.collection.as_ref().unwrap().verified==true

    )]
    pub metadata: Account<'info, MetadataAccount>,

    /// Master edition account for the NFT (required for freezing).
    #[account(
        seeds=[b"metadata", metadata_program.key().as_ref(),nft_mint.key().as_ref(),b"edition"],
        seeds::program=metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds=[b"stake_config"],
        bump=stake_config.stake_config_bump
    )]
    pub stake_config: Account<'info, StakeConfig>,

    #[account(
        mut,
        seeds=[b"user_account", staker.key().as_ref()],
        bump=staker_account.staker_user_acct_bump
    )]
    pub staker_account: Account<'info, StakerUserAccount>,

    #[account(
        init,
        payer=staker,
        space=8+StakeAccount::INIT_SPACE,
        seeds=[b"stake", nft_mint.key().as_ref(),stake_config.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    pub token_prgram: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}
impl<'info> Stake<'info> {
    /// Executes the staking logic.
    ///
    /// - Checks that the user has not exceeded the max number of staked NFTs.
    /// - Creates a `StakeAccount` to track the staked NFT.
    /// - Approves the stake account as a delegate of the user's NFT token account.
    /// - Freezes the NFT via the Metaplex Token Metadata program, preventing transfers.
    /// - Increments the user's total staked count in `UserAccount`.
    ///
    /// The NFT becomes locked until an `unstake` instruction is called.
    pub fn stake(&mut self, bumps: &StakeBumps) -> Result<()> {
        require!(
            self.staker_account.amount_staked < self.stake_config.max_stake,
            StakeError::MaxStakeReached
        );
        self.stake_account.set_inner(StakeAccount {
            owner: self.staker.key(),
            mint: self.nft_mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            stake_acct_bump: bumps.stake_account,
        });

        self.approve_nft_authority()?;

        self.freeze_nft_account()?;

        self.staker_account.amount_staked += 1;

        Ok(())
    }

    pub fn approve_nft_authority(&mut self)->Result<()> {
        let cpi_program = self.token_prgram.to_account_info();

        let approve_accounts = Approve {
            to: self.staker_mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.staker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, approve_accounts);
        approve(cpi_ctx, 1)?;
        Ok(())
    }

    pub fn freeze_nft_account(&mut self) -> Result<()>{
        let signers_seeds = &[
            b"stake",
            self.nft_mint.to_account_info().key.as_ref(),
            self.stake_config.to_account_info().key.as_ref(),
            &[self.stake_account.stake_acct_bump],
        ];
        let signers_seeds = &[&signers_seeds[..]];

        let (delegate, token_account, edition, nft_mint, token_program, metadata_program) = {
            (
                &self.stake_account.to_account_info(),
                &self.staker_mint_ata.to_account_info(),
                &self.master_edition.to_account_info(),
                &self.nft_mint.to_account_info(),
                &self.token_prgram.to_account_info(),
                &self.metadata_program.to_account_info(),
            )
        };

        let freeze_accounts = FreezeDelegatedAccountCpiAccounts {
            delegate,
            token_account,
            edition,
            mint: nft_mint,
            token_program,
        };

        FreezeDelegatedAccountCpi::new(metadata_program, freeze_accounts)
            .invoke_signed(signers_seeds)?;
        Ok(())
    }
}
