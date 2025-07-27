use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Marketplace {
    pub admin: Pubkey,
    /// basis points
    pub fee: u16,
    #[max_len(32)]
    pub name: String,
    pub treasury_bump: u8,
    pub rewards_bump: u8,
    pub marketplace_bump: u8,
}
