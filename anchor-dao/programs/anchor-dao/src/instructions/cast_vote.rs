use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::{Dao, Proposal, Vote};


/// Casts a vote on a proposal within a DAO using quadratic voting.
/// 
/// Quadratic voting means that the influence of a vote is the square root of the number of tokens held by the voter.
/// This allows token holders with fewer tokens to still have meaningful input while limiting the domination of large holders.

#[derive(Accounts)]
pub struct CastVote<'info> {
      /// The voter casting the vote.
    /// Must sign the transaction and pay for the `vote_account` rent.
    #[account(mut)]
    pub voter: Signer<'info>,

    #[account(
        seeds=[
            b"dao", 
            dao_account_pda.authority.as_ref(),
            dao_account_pda.name.as_bytes().as_ref()
        ],
        bump=dao_account_pda.bump
    )]
    pub dao_account_pda: Account<'info, Dao>,

    #[account(
        mut,
        seeds=[
            b"proposal", 
            dao_account_pda.key().as_ref(), 
            dao_account_pda.proposal_count.to_le_bytes().as_ref()
        ],
        bump=proposal_account.bump
    )]
    pub proposal_account: Account<'info, Proposal>,

    #[account(
        init,
        payer= voter,
        space= 8+Vote::INIT_SPACE,
        seeds=[b"vote", proposal_account.key().as_ref(), voter.key().as_ref()], 
        bump
    )]
    pub vote_account: Account<'info, Vote>,

    #[account(
        token::authority=voter
    )]
    pub voter_token_account: Account<'info,TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}


impl <'info> CastVote<'info> {
    /// Casts the vote using quadratic voting.
    ///
    /// - `vote_type`: A `u8` indicating the type of vote (e.g., 0 = no, 1 = yes).
    /// - `bumps`: The bump seeds for the involved PDAs (only `vote_account` is used here).
    ///
    /// Calculates the square root of the token amount in the voter’s account to determine voting credits.
    /// This is a simplified approach to implement quadratic voti
    pub fn cast_vote(&mut self, vote_type:u8, bumps: &CastVoteBumps)->Result<()> {
        let voting_credits=(self.voter_token_account.amount as f64).sqrt() as u64;

        self.vote_account.set_inner(
           Vote { voter: self.voter.key(), vote_type, vote_credits: voting_credits, bump: bumps.vote_account }
        );
        Ok(())
    }
}