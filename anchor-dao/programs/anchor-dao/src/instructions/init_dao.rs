use anchor_lang::prelude::*;

use crate::Dao;

/// Accounts context for initializing a new DAO.
///
/// This instruction creates and initializes a `Dao` account using the authority (caller)
/// and a user-provided name. The DAO account is a PDA derived using the authority's public key
/// and the name string, making it unique per authority-name pair.
///
/// ### PDA Derivation:
/// `seeds = [b"dao", authority_pubkey, name_bytes]`
///
/// This ensures the same authority can create multiple DAOs as long as the name differs.

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
    /// Initializes the DAO with the provided name and the caller as the authority.
    ///
    /// Sets:
    /// - `name` of the DAO
    /// - `authority` to the signer
    /// - `proposal_count` to zero
    /// - `bump` to the PDA bump seed
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
