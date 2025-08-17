use anchor_lang::prelude::*;
use mpl_core::{
    accounts::BaseCollectionV1,
    instructions::CreateV2CpiBuilder,
    types::{Attribute, Attributes, Plugin, PluginAuthorityPair},
   
};

use crate::{
    error::MarketplaceError, DropCampaign, MarketplaceConfig, SupporterAccount,
    CONFIG_BINARY_STRING, DROP_CAMPAIGN_BINARY_STRING, SUPPORTER_BINARY_STRING,
};

use mpl_core::ID as MPL_CORE_ID;


#[derive(Accounts)]
pub struct MintSbt<'info> {
    #[account(mut)]
    pub supporter: Signer<'info>,

    #[account(
        seeds=[CONFIG_BINARY_STRING, marketplace_config.authority.key().as_ref()],
        bump= marketplace_config.bump
    )]
    pub marketplace_config: Account<'info, MarketplaceConfig>,

    #[account(
        mut,
        seeds=[
            DROP_CAMPAIGN_BINARY_STRING,
            marketplace_config.key().as_ref(),
            drop_campaign.creator.key().as_ref(), 
            drop_campaign.name.as_bytes().as_ref()
        ],
        bump=drop_campaign.bump
    )]
    pub drop_campaign: Account<'info, DropCampaign>,

    /// CHECK; THE ADDRESS IS ALREADY CHECKED IN THE CONSTARINT AND MPL CORE
    #[account(
        mut,
        constraint= drop_campaign.collection_mint.key()==collection_mint.key() @MarketplaceError::Unauthorized
    )]
    pub collection_mint: AccountInfo<'info>,

    #[account(
        mut,
        close=supporter,
        seeds=[SUPPORTER_BINARY_STRING,drop_campaign.key().as_ref(),supporter.key().as_ref()],
        bump=supporter_account.bump
    )]
    pub supporter_account: Account<'info, SupporterAccount>,

    #[account(mut)]
    pub supporter_asset_nft: Signer<'info>,

    /// CHECK; THE ADDRESS IS ALREADY PASSED IN THE CONSTARINT
    #[account(address=MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateAssetArgs {
    pub name: String,
    pub uri: String,
}

impl<'info> MintSbt<'info> {
    pub fn mint_sbt_for_suporter(&mut self, args: CreateAssetArgs) -> Result<()> {
        require_keys_eq!(
            self.supporter_account.authority.key(),
            self.supporter.key(),
            MarketplaceError::Unauthorized
        );
        require!(
            self.drop_campaign.is_finalized && self.drop_campaign.is_successful,
            MarketplaceError::CampaignNotSuccessful
        );
        require!(
            !self.supporter_account.has_minted_sbt,
            MarketplaceError::AlreadyMinted
        );

        let drop_name = &self.drop_campaign.name;
        let creator_key = &self.drop_campaign.creator.key();
        let units_ordered = self.supporter_account.units_ordered;
        let total_spent_on_campaign = self.supporter_account.amount_paid_to_campaign_vault;

        

        let base_collection = {
            let mut drop_collection_data = &self.collection_mint.data.borrow()[..];
            BaseCollectionV1::deserialize(&mut drop_collection_data)?
        };

        require!(
            base_collection.num_minted as u64 <= self.drop_campaign.supporter_count,
            MarketplaceError::CollectionFull
        );
        let mut drop_collection_plugin: Vec<PluginAuthorityPair> = vec![];

        let attribute_list = vec![
            Attribute {
                key: "Drop Name".to_string(),
                value: drop_name.clone(),
            },
            Attribute {
                key: "Creator".to_string(),
                value: creator_key.to_string(),
            },
            Attribute {
                key: "Total spent".to_string(),
                value: total_spent_on_campaign.to_string(),
            },
            Attribute {
                key: "Units Ordered".to_string(),
                value: units_ordered.to_string(),
            },
            Attribute {
                key: "NFT Number".to_string(),
                value: format!(
                    "#{} / {}",
                    base_collection
                        .num_minted
                        .checked_add(1)
                        .unwrap()
                        .to_string(),
                    self.drop_campaign.supporter_count.to_string()
                ),
            },
        ];

        drop_collection_plugin.push(PluginAuthorityPair {
            plugin: Plugin::Attributes(Attributes { attribute_list }),
            authority: None,
        });

    //    drop_collection_plugin.push(
    //     PluginAuthorityPair {
    //         plugin: Plugin::,
    //         authority: Some(mpl_core::types::PluginAuthority::Address {
    //             address: self.drop_campaign.key(),
    //         }),
    //     }
    //    );

        let signers_seeds = &[
            b"drop_campaign",
            self.marketplace_config.to_account_info().key.as_ref(),
            self.drop_campaign.creator.as_ref(),
            self.drop_campaign.name.as_bytes().as_ref(),
            &[self.drop_campaign.bump],
        ];

        let signers_seeds = &[&signers_seeds[..]];

        CreateV2CpiBuilder::new(self.mpl_core_program.as_ref())
            .asset(self.supporter_asset_nft.to_account_info().as_ref())
            .collection(Some(self.collection_mint.to_account_info().as_ref()))
            .payer(self.supporter.to_account_info().as_ref())
            .authority(Some(self.drop_campaign.to_account_info().as_ref()))
            .owner(Some(self.supporter.to_account_info().as_ref()))
            .system_program(self.system_program.to_account_info().as_ref())
            .name(args.name)
            .uri(args.uri)
            .plugins(drop_collection_plugin)
            .invoke_signed(signers_seeds)?;

        self.supporter_account.has_minted_sbt = true;
        Ok(())
    }
}
