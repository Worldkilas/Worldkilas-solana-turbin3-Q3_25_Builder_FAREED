pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("42WyLUdYjx1t9czGdEFV5y5ECSDnnggfTLk3shbJYLdK");

#[program]
pub mod anchor_dao {
    use super::*;

    pub fn initialize_dao(ctx: Context<InitializeDao>, name: String) -> Result<()> {
        ctx.accounts.init_dao(name, &ctx.bumps)
    }

    pub fn initialize_proposal(ctx: Context<InitializeProposal>, metadata: String) -> Result<()> {
        ctx.accounts.init_proposal(metadata, &ctx.bumps)
    }

    pub fn cast_vote(ctx: Context<CastVote>, vote_type: u8) -> Result<()> {
        ctx.accounts.cast_vote(vote_type, &ctx.bumps)
    }
}
