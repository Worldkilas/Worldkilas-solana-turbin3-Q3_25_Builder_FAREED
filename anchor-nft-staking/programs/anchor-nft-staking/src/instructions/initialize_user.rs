use anchor_lang::prelude::*;
use crate::StakerUserAccount;

#[derive(Accounts)]
pub struct InitializeUser<'info>{
    #[account(mut)]
    pub user:Signer<'info>,

    #[account(
        init,
        payer=user,
        seeds=[b"user_account",user.key().as_ref()],
        bump,
        space=8+StakerUserAccount::INIT_SPACE 

    )]
    pub user_account: Account<'info, StakerUserAccount>,
     
    
    pub system_program: Program<'info, System>
}

impl <'info> InitializeUser<'info> {
    pub fn init_user_account(&mut self,bumps:&InitializeUserBumps)->Result<()>{
        self.user_account.set_inner(StakerUserAccount { points: 0, amount_staked: 0, staker_user_acct_bump: bumps.user_account });
        Ok(())
    }
}