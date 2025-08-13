use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DropCampaign {
    pub creator: Pubkey,
    #[max_len(32)]
    pub name: String,
    pub goal_supporters: u32,
    pub pledged_supporters: u32,
    pub price: u64,
    pub is_finalized: bool,
    pub is_successful: bool,
    pub start_timestamp: i64,
    pub end_timestamps: i64,
    pub collection_mint: Pubkey,
    pub vault_bump: u8,
    pub bump: u8,
}
