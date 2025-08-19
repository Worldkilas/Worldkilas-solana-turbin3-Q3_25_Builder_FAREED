use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::MarketplaceError, DropCampaign, MarketplaceConfig, SupporterAccount};


/// Instruction: ClaimRefund
/// 
/// Allows a supporter to claim back their committed funds if:
/// - The campaign has been finalized, **and**
/// - The campaign was not successful (goal not reached).
/// 
/// Once executed:
/// - Funds are transferred from the campaign vault back to the supporter's ATA.
/// - The supporter account is closed and lamports returned.
/// - The supporter is marked as refunded to prevent double refunds.
#[derive(Accounts)]

pub struct ClaimRefund<'info> {
    #[account(mut)]
    pub supporter: Signer<'info>,

     /// Global marketplace config (authority + bump)
    #[account(
        seeds=[b"config", marketplace_config.authority.key().as_ref()],
        bump= marketplace_config.bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        mut,
        seeds=[
            b"drop_campaign",
            marketplace_config.key().as_ref(),
            drop_campaign.creator.key().as_ref(), 
            drop_campaign.name.as_bytes().as_ref()
        ],
        bump=drop_campaign.bump
    )]
    pub drop_campaign: Account<'info, DropCampaign>,

    /// Token mint used for contributions
    pub token_mint: InterfaceAccount<'info, Mint>,

     /// Supporter account PDA tracking their contribution
    /// - Closed after refund to reclaim rent
    #[account(
        mut,
        close=supporter,
        seeds=[b"supporter",drop_campaign.key().as_ref(),supporter.key().as_ref()],
        bump=supporter_account.bump,
        constraint= supporter.key()== supporter_account.authority.key() @MarketplaceError::Unauthorized
    )]
    pub supporter_account: Account<'info, SupporterAccount>,

    #[account(
       mut,
        associated_token::mint= token_mint,
        associated_token::authority=supporter,
    )]
    pub supporter_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=drop_campaign
    )]
    pub campaign_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimRefund<'info> {
    /// Executes the refund flow:
    /// 1. Ensures campaign was finalized but unsuccessful
    /// 2. Ensures supporter hasn’t already been refunded
    /// 3. Transfers funds from campaign vault → supporter ATA
    /// 4. Marks supporter as refunded
    pub fn claim_refund(&mut self) -> Result<()> {
        require!(
            self.drop_campaign.is_finalized && !self.drop_campaign.is_successful,
            MarketplaceError::CampaignSuccessful
        );
        require!(
            self.supporter_account.is_refunded,
            MarketplaceError::AlreadyRefunded
        );

        let signer_seeds = &[
            b"drop_campaign",
            self.marketplace_config.to_account_info().key.as_ref(),
            self.drop_campaign.creator.as_ref(),
            self.drop_campaign.name.as_bytes().as_ref(),
            &[self.drop_campaign.bump],
        ];

        let signer_seeds = &[&signer_seeds[..]];

        let accounts = TransferChecked {
            from: self.campaign_vault.to_account_info(),
            to: self.supporter_token_account.to_account_info(),
            authority: self.drop_campaign.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer_checked(
            cpi_ctx,
            self.supporter_account.amount_paid_to_campaign_vault,
            self.token_mint.decimals,
        )?;

        self.supporter_account.is_refunded = true;

        Ok(())
    }
}
