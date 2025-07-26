use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakerUserAccount {
    pub points: u32,
    pub amount_staked: u8,
    pub staker_user_acct_bump: u8,
}
