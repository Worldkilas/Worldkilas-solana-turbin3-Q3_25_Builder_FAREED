use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SupporterAccount {
    pub authority: Pubkey,
    pub drop_campaign: Pubkey,
    pub amount_paid: u64,
    pub refunded: bool,
    pub sbt_minted: bool,
    pub bump: u8,
}
