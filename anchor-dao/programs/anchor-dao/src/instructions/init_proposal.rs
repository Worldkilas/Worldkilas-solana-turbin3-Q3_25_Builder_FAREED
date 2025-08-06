use anchor_lang::prelude::*;

use crate::{Dao, Proposal};

#[derive(Accounts)]
pub struct InitializeProposal<'info> {
    #[account(mut)]
    pub dao_authority: Signer<'info>,

    #[account(
        mut,
        seeds=[b"dao", dao_authority.key().as_ref(), dao_account_pda.name.as_bytes().as_ref()],
        bump=dao_account_pda.bump
    )]
    pub dao_account_pda: Account<'info, Dao>,

    #[account(
        init,
        payer=dao_authority,
        space= 8+Proposal::INIT_SPACE,
        seeds=[
            b"proposal",
            dao_account_pda.key().as_ref(),
            dao_account_pda.proposal_count.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub proposal_account: Account<'info, Proposal>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeProposal<'info> {
    pub fn init_proposal(
        &mut self,
        metadata: String,
        bumps: &InitializeProposalBumps,
    ) -> Result<()> {
        self.dao_account_pda.proposal_count+=1;
        self.proposal_account.set_inner(Proposal {
            metadata,
            authority: self.dao_authority.key(),
            yes_vote_count: 0,
            no_vote_count: 0,
            bump: bumps.proposal_account,
        });
        Ok(())
    }
}
