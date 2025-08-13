use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketplaceConfig {
    pub authority: Pubkey,
    pub fee_bps: u16,
    pub treasury_bump: u8,
    pub bump: u8,
}
