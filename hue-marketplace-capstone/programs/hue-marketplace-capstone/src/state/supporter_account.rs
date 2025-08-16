use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct SupporterAccount {
    pub authority: Pubkey,
    pub drop_campaign: Pubkey,
    pub amount_paid_to_campaign_vault: u64,
    pub refunded: bool,
    pub has_minted_sbt: bool,
    pub units_ordered: u32,
    pub bump: u8,
}
