use std::vec;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use mpl_core::types::{Attribute, Attributes, Plugin, PluginAuthorityPair};
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder,
    types::{
        ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, OracleInitInfo,
    },
    ID as MPL_CORE_ID,
};

use crate::{
    DropCampaign, MarketplaceConfig, CONFIG_BINARY_STRING, DROP_CAMPAIGN_BINARY_STRING,
    ONCHAIN_METAPLEX_ORACLE_PLUGIN,
};


/// Instruction: InitializeCampaign
///
/// Creates and initializes a new drop campaign:
/// - Sets campaign metadata (goal, price, duration, etc.)
/// - Creates the campaign vault (ATA owned by the campaign PDA)
/// - Registers the campaign with Metaplex Core via CPI
/// - Attaches oracle plugin for lifecycle validation
///
/// Once executed, supporters can begin committing funds to this campaign.


#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitializeCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(mut)]
    pub collection_mint: Signer<'info>,

    #[account(
        seeds=[CONFIG_BINARY_STRING, marketplace_config.authority.key().as_ref()],
        bump= marketplace_config.bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        init,
        payer=creator,
        space=8+DropCampaign::INIT_SPACE,
        seeds=[
            DROP_CAMPAIGN_BINARY_STRING,
            marketplace_config.key().as_ref(),
            creator.key().as_ref(), 
            name.as_bytes().as_ref()
        ],
        bump
    )]
    pub drop_campaign: Account<'info, DropCampaign>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    /// Campaign vault (ATA owned by the campaign PDA)
    /// Stores funds contributed by supporters until campaign finalization
    #[account(
        init,
        payer= creator,
        associated_token::mint= token_mint,
        associated_token::authority=drop_campaign,
    )]
    pub campaign_vault: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: THIS IS CHECKED BY THE ADDRESS CONSTRAINT
    #[account(address= MPL_CORE_ID)]
    pub mpl_core_program: AccountInfo<'info>,

    /// CHECK: THIS IS CHECKED BY THE MPL CORE PROGRAM
    #[account(address=ONCHAIN_METAPLEX_ORACLE_PLUGIN)]
    pub oracle_account: AccountInfo<'info>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

/// Arguments provided by the creator when initializing a drop campaign
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitDropCampaignArgs {
    pub name: String,
    pub goal_orders: u32,
    pub pledged_orders: u32,
    pub allowed_units_per_supporter: u32,
    pub price: u64,
    pub start_timestamp: i64,
    pub days_until_end: u64,
    pub uri: String,
}

impl<'info> InitializeCampaign<'info> {
    /// Launches a new drop campaign.
    /// 1. Calculates campaign timeline
    /// 2. Initializes the DropCampaign PDA
    /// 3. Calls Metaplex Core CPI to create a collection
    /// 4. Attaches oracle plugin for transfer validation
    pub fn launch_drop(
        &mut self,
        args: InitDropCampaignArgs,
        bumps: &InitializeCampaignBumps,
    ) -> Result<()> {
        let start_timestamp = Clock::get()?.unix_timestamp;
        let end_timestamp = start_timestamp + (args.days_until_end as i64 * 24 * 60 * 60);

        let attribute_list = vec![
            Attribute {
                key: "Drop name".to_string(),
                value: args.name.clone(),
            },
            Attribute {
                key: "Price".to_string(),
                value: args.price.to_string(),
            },
        ];

        let drop_name = args.name.clone();
        self.drop_campaign.set_inner(DropCampaign {
            creator: self.creator.key(),
            name: args.name,
            goal_orders: args.goal_orders,
            pledged_orders: args.pledged_orders,
            allowed_units_per_supporter: args.allowed_units_per_supporter,
            price: args.price,
            is_finalized: false,
            is_successful: false,
            start_timestamp,
            end_timestamp,
            collection_mint: self.collection_mint.key(),
            supporter_count: 0,
            bump: bumps.drop_campaign,
        });

        CreateCollectionV2CpiBuilder::new(self.mpl_core_program.as_ref())
            .collection(self.collection_mint.as_ref())
            .payer(self.creator.as_ref())
            .name(drop_name)
            .uri(args.uri)
            // .plugins(vec![PluginAuthorityPair {
            //     plugin: Plugin::Attributes(Attributes { attribute_list }),
            //     authority: None,
            // }])
            .system_program(self.system_program.to_account_info().as_ref())
            .external_plugin_adapters(vec![ExternalPluginAdapterInitInfo::Oracle(
                OracleInitInfo {
                    base_address: ONCHAIN_METAPLEX_ORACLE_PLUGIN,
                    init_plugin_authority: None,
                    lifecycle_checks: vec![(
                        HookableLifecycleEvent::Transfer,
                        ExternalCheckResult { flags: 4 },
                    )],
                    base_address_config: None,
                    results_offset: Some(mpl_core::types::ValidationResultsOffset::Anchor),
                },
            )])
            .invoke()?;

        msg!("✅ Builder constructed, invoking CPI...");

        msg!("🎉 Collection successfully created");

        Ok(())
    }
}
