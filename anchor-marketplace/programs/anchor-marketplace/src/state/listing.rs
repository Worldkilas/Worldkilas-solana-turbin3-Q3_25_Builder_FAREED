use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Listing{
    pub seller: Pubkey,
    pub seller_mint: Pubkey,
    pub listing_bump: u8,
    pub price: u64
}