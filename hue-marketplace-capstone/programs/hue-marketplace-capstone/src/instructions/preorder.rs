use anchor_lang::{prelude::*, system_program::Transfer};
use anchor_spl::{associated_token::AssociatedToken, token::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{error::MarketplaceError, require_campaign_active, require_campaign_not_finalized, DropCampaign, MarketplaceConfig, SupporterAccount};

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
        associated_token::mint=token_mint,
        associated_token::authority=drop_campaign
    )]
    pub campaign_vault: InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Preorder<'info> {
    pub fn init_supporter_account(&mut self, amount: u64, bumps: &PreorderBumps) -> Result<()> {
        self.supporter_account.set_inner(SupporterAccount {
            authority: self.supporter.key(),
            drop_campaign: self.drop_campaign.key(),
            amount_paid: amount,
            refunded: false,
            bump: bumps.supporter_account,
            sbt_minted: false,
        });
        Ok(())
    }

    pub fn commit_funds(&mut self, amount_to_commit: u64)->Result<()>{
        let now= Clock::get()?.unix_timestamp;
        require_campaign_active!(
            now,
            self.drop_campaign
        );
        require_campaign_not_finalized!(self.drop_campaign.is_finalized);
        require!(amount_to_commit< self.drop_campaign.price, MarketplaceError::InvalidPrice);


        let cpi_accounts= TransferChecked{
            from: self.supporter_token_account.to_account_info(),
            to: self.campaign_vault.to_account_info(),
            authority: self.supporter.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let cpi_ctx=CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, amount_to_commit, self.token_mint.decimals)?;
        Ok(())
    }
}
