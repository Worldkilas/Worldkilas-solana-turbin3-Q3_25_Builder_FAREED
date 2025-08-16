use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub commit_fees_bps: u16,
    pub withdraw_fees_bps: u16,
    pub treasury_bump: u8,
    pub bump: u8,
}
