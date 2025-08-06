use anchor_lang::prelude::*;

#[account]
#[derive(Debug, InitSpace)]
pub struct Dao {
    #[max_len(300)]
    pub name: String,
    pub authority: Pubkey,
    pub proposal_count: u64,
    pub bump: u8,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Proposal {
    #[max_len(300)]
    pub metadata: String,
    pub authority: Pubkey,
    pub yes_vote_count: u64,
    pub no_vote_count: u64,
    pub bump: u8,
}

#[account]
#[derive(Debug, InitSpace)]
pub struct Vote {
    pub voter: Pubkey,
    pub vote_type: u8,
    pub vote_credits: u64,
    pub bump: u8
}
