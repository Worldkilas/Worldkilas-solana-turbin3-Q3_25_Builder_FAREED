use anchor_lang::prelude::*;

use crate::Dao;

#[derive(Accounts)]
#[instruction(name:String)]
pub struct InitializeDao<'info> {
    #[account(mut)]
    pub dao_authority: Signer<'info>,

    #[account(
        init,
        payer=dao_authority,
        space= 8+Dao::INIT_SPACE,
        seeds=[b"dao",dao_authority.key().as_ref(),name.as_bytes().as_ref()],
        bump
    )]
    pub dao_account_pda: Account<'info, Dao>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeDao<'info> {
    pub fn init_dao(&mut self, name: String, bumps: &InitializeDaoBumps) -> Result<()> {
        self.dao_account_pda.set_inner(Dao {
            name,
            authority: self.dao_authority.key(),
            proposal_count: 0,
            bump: bumps.dao_account_pda,
        });
        Ok(())
    }
}
