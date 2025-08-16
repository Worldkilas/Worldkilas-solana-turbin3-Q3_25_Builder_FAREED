use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DropCampaign {
    pub creator: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub goal_orders: u32,
    pub pledged_orders: u32,
    pub allowed_units_per_supporter: u32,
    pub price: u64,
    pub is_finalized: bool,
    pub is_successful: bool,
    pub start_timestamp: i64,
    pub end_timestamp: i64,
    pub collection_mint: Pubkey,
    pub supporter_count: u64,
    pub bump: u8,
}
