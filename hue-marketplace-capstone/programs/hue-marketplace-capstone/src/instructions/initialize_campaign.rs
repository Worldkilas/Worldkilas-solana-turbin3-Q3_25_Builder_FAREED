use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{DropCampaign, MarketplaceConfig};

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        seeds=[b"config", marketplace_config.authority.key().as_ref()],
        bump= marketplace_config.bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        init,
        payer=creator,
        space=8+DropCampaign::INIT_SPACE,
        seeds=[
            b"drop_campaign",
            marketplace_config.key().as_ref(),
            creator.key().as_ref(), 
            name.as_bytes().as_ref()
        ],
        bump
    )]
    pub drop_campaign: Account<'info, DropCampaign>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer= creator,
        associated_token::mint= token_mint,
        associated_token::authority=drop_campaign,
    )]
    pub campaign_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeCampaign<'info> {
    // pub fn launch_campaign(&mut self) -> Result<()> {
    //     self.drop_campaign.set_inner(
    //         DropCampaign { creator: self.creator.key(), name, goal_supporters, pledged_supporters: 0, collect, price: (), finalized: (), succesful: (), start_timestamp: (), end_timestamps: (), vault_bump: (), bump: () }
    //     );
    //     Ok(())
    // }
}
