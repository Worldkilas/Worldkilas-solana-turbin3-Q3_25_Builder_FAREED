use anchor_lang::{prelude::*, solana_program::example_mocks::solana_sdk::sysvar::fees};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    require_campaign_finalized_and_successful, DropCampaign, MarketplaceConfig, BASIS_FEE_POINTS,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds=[b"config", marketplace_config.authority.key().as_ref()],
        bump= marketplace_config.bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        mut,
        has_one= creator,
        seeds=[
            b"drop_campaign",
            marketplace_config.key().as_ref(),
            drop_campaign.creator.key().as_ref(), 
            drop_campaign.name.as_bytes().as_ref()
        ],
        bump=drop_campaign.bump
    )]
    pub drop_campaign: Account<'info, DropCampaign>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer= creator,
        associated_token::mint= token_mint,
        associated_token::authority=creator,
    )]
    pub creator_token_account: InterfaceAccount<'info, TokenAccount>,

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

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require_campaign_finalized_and_successful!(self.drop_campaign);

        let fees = self.marketplace_config.withdraw_fees_bps;

        let fees = (self.campaign_vault.amount as u64 * fees as u64) / BASIS_FEE_POINTS;

        self.deposit_market_fees(fees)?;

        let amount_to_creator = self.campaign_vault.amount - fees;

        let transfer_accounts = TransferChecked {
            from: self.campaign_vault.to_account_info(),
            to: self.creator_token_account.to_account_info(),
            authority: self.drop_campaign.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let signer_seeds = &[
            b"drop_campaign",
            self.marketplace_config.to_account_info().key.as_ref(),
            self.creator.to_account_info().key.as_ref(),
            self.drop_campaign.name.as_bytes().as_ref(),
            &[self.drop_campaign.bump],
        ];

        let signer_seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, amount_to_creator, self.token_mint.decimals)?;
        self.close_campaign_vault()?;
        Ok(())
    }

    fn deposit_market_fees(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.campaign_vault.to_account_info(),
            to: self.treasury_ata.to_account_info(),
            authority: self.drop_campaign.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let signer_seeds = &[
            b"drop_campaign",
            self.marketplace_config.to_account_info().key.as_ref(),
            self.creator.to_account_info().key.as_ref(),
            self.drop_campaign.name.as_bytes().as_ref(),
            &[self.drop_campaign.bump],
        ];

        let signer_seeds = &[&signer_seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_ctx, amount, self.token_mint.decimals)?;

        Ok(())
    }

    fn close_campaign_vault(&mut self) -> Result<()> {
        let accounts = CloseAccount {
            account: self.campaign_vault.to_account_info(),
            destination: self.creator.to_account_info(),
            authority: self.drop_campaign.to_account_info(),
        };
        let signer_seeds = &[
            b"drop_campaign",
            self.marketplace_config.to_account_info().key.as_ref(),
            self.creator.to_account_info().key.as_ref(),
            self.drop_campaign.name.as_bytes().as_ref(),
            &[self.drop_campaign.bump],
        ];
        let signer_seeds = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        close_account(cpi_ctx)?;
        Ok(())
    }
}
