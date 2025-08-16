use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::MarketplaceError, require_campaign_active, require_campaign_not_finalized, DropCampaign,
    MarketplaceConfig, SupporterAccount, BASIS_FEE_POINTS,
};

#[derive(Accounts)]
pub struct Preorder<'info> {
    #[account(mut)]
    pub supporter: Signer<'info>,

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

    #[account(
        init,
        payer= supporter,
        space= 8+ SupporterAccount::INIT_SPACE,
        seeds=[b"supporter",drop_campaign.key().as_ref(),supporter.key().as_ref()],
        bump
    )]
    pub supporter_account: Account<'info, SupporterAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint= token_mint,
        associated_token::authority=supporter,
    )]
    pub supporter_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"treasury", marketplace_config.key().as_ref()],
        bump=marketplace_config.treasury_bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint=token_mint,
        associated_token::authority=treasury 
    )]
    pub treasury_ata: InterfaceAccount<'info, TokenAccount>,

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

impl<'info> Preorder<'info> {
    pub fn preorder(&mut self, units_ordered: u32, bumps: &PreorderBumps) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require_campaign_active!(now, self.drop_campaign);
        require_campaign_not_finalized!(self.drop_campaign.is_finalized);
        // require!(
        //     price_per_unit > 0 && price_per_unit as u64 == self.drop_campaign.price,
        //     MarketplaceError::InvalidPrice
        // );

        let is_first_time = self.supporter_account.amount_paid_to_campaign_vault == 0
            && self.supporter_account.units_ordered == 0;

        let total_amount = self.drop_campaign.price * units_ordered as u64;
        require!(
            self.marketplace_config.commit_fees_bps as u64 <= BASIS_FEE_POINTS,
            MarketplaceError::InvalidFeePoints
        );
        let marketplace_fees_amount = total_amount
            .checked_mul(self.marketplace_config.commit_fees_bps as u64)
            .ok_or(MarketplaceError::Overflow)?
            .checked_div(BASIS_FEE_POINTS)
            .ok_or(MarketplaceError::Overflow)?;

        let amount_to_commit = total_amount - marketplace_fees_amount;

        if is_first_time {
            require!(
                units_ordered > 0
                    && units_ordered <= self.drop_campaign.allowed_units_per_supporter,
                MarketplaceError::InvalidUnitsOrdered
            );
            self.init_supporter_account(amount_to_commit, units_ordered, bumps.supporter_account)?;
            self.drop_campaign.supporter_count += 1;
        } else {
            require!(
                self.supporter_account.units_ordered + units_ordered
                    <= self.drop_campaign.allowed_units_per_supporter,
                MarketplaceError::InvalidUnitsOrdered
            );
            self.supporter_account.amount_paid_to_campaign_vault += amount_to_commit;
            self.supporter_account.units_ordered += units_ordered;
        }
        self.drop_campaign.pledged_orders += units_ordered;

        self.pay_marketplace_fees(marketplace_fees_amount)?;

        self.commit_funds_to_campaign(amount_to_commit)?;

        if self.drop_campaign.pledged_orders==self.drop_campaign.goal_orders {
            self.finalize_campaign()?;
        }

        Ok(())
    }

    fn init_supporter_account(&mut self, amount: u64, units_ordered: u32, bump: u8) -> Result<()> {
        self.supporter_account.set_inner(SupporterAccount {
            authority: self.supporter.key(),
            drop_campaign: self.drop_campaign.key(),
            amount_paid_to_campaign_vault: amount,
            refunded: false,
            has_minted_sbt: false,
            units_ordered,
            bump,
        });

        Ok(())
    }

    fn commit_funds_to_campaign(&mut self, amount_to_commit: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.supporter_token_account.to_account_info(),
            to: self.campaign_vault.to_account_info(),
            authority: self.supporter.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount_to_commit, self.token_mint.decimals)?;
        Ok(())
    }

    fn pay_marketplace_fees(&mut self, fees_amount: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.supporter_token_account.to_account_info(),
            to: self.treasury_ata.to_account_info(),
            authority: self.supporter.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, fees_amount, self.token_mint.decimals)?;

        Ok(())
    }

    fn finalize_campaign(&mut self)->Result<()> {
        require!(!self.drop_campaign.is_finalized, MarketplaceError::AlreadyFinalized);

        let now= Clock::get()?.unix_timestamp;
        require!(now>=self.drop_campaign.end_timestamp, MarketplaceError::TooEarlyToFinalize);

        self.drop_campaign.is_finalized=true;

        self.drop_campaign.is_successful=self.drop_campaign.pledged_orders==self.drop_campaign.goal_orders;

        Ok(())
    }
}
