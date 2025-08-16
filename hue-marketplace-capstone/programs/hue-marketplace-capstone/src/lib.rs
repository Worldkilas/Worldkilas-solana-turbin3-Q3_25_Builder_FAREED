pub mod constants;
pub mod error;
pub mod helper_macros;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use helper_macros::*;
pub use instructions::*;
pub use state::*;

declare_id!("4V8C51KJ45atM2PBmYp91N2tQLHbzMThvMHvvzFKzAQT");

#[program]
pub mod hue_marketplace_capstone {

    use super::*;

    pub fn initialize_marketplace(
        ctx: Context<InitializeMarketplace>,
        withdraw_fees: u16,
        commit_fees: u16,
    ) -> Result<()> {
        ctx.accounts
            .init_marketplace(commit_fees, withdraw_fees, &ctx.bumps)
    }

    pub fn initialize_campaign(
        ctx: Context<InitializeCampaign>,
        args: InitDropCampaignArgs,
    ) -> Result<()> {
        ctx.accounts.launch_drop(args, &ctx.bumps)
    }

    pub fn preorder(ctx: Context<Preorder>, units_ordered: u32) -> Result<()> {
        ctx.accounts.preorder(units_ordered, &ctx.bumps)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()
    }

    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        ctx.accounts.claim_refund()
    }

    pub fn mint_sbt_for_suporter(ctx: Context<MintSbt>, args: CreateAssetArgs) -> Result<()> {
        ctx.accounts.mint_sbt_for_suporter(args)
    }
}
