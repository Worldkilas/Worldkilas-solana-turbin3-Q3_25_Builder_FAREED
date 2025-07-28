use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    solana_program::{self, ed25519_program, sysvar::instructions::load_instruction_at_checked},
    system_program::{transfer, Transfer},
};
use solana_program::hash::hash;

use crate::{error::DiceError, Bet};

pub const HOUSE_EDGE: u16 = 150; //1.5% basis points
#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,
    ///CHECK: THis is safe
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        close=player,
        has_one=player,
        seeds=[b"bet", vault.key().as_ref(), bet.seed.to_be_bytes().as_ref()],
        bump=bet.bump
    )]
    pub bet: Account<'info, Bet>,

    #[account(
        address=solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar)?;
        require_keys_eq!(
            ix.program_id,
            ed25519_program::ID,
            DiceError::Ed25519Program,
        );
        require_eq!(ix.accounts.len(), 0, DiceError::Ed25519Accounts);

        let ed25519_signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(ed25519_signatures.len(), 0, DiceError::Ed25519DataLength,);

        let ed25519_signature = &ed25519_signatures[0];

        require!(ed25519_signature.is_verifiable, DiceError::Ed25519Header);

        require_keys_eq!(
            ed25519_signature
                .public_key
                .ok_or(DiceError::Ed25519Pubkey)?,
            self.house.key(),
            DiceError::Ed25519Pubkey
        );

        require!(
            ed25519_signature
                .signature
                .ok_or(DiceError::Ed25519Signature)?
                .eq(sig),
            DiceError::Ed25519Signature
        );

        require!(
            ed25519_signature
                .message
                .as_ref()
                .ok_or(DiceError::Ed25519Signature)?
                .eq(&self.bet.to_slice()),
            DiceError::Ed25519Signature
        );
        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        let hash = hash(sig).to_bytes();
        let mut hash_16: [u8; 16] = [0; 16];

        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payout = (self.bet.bet_amount as u128)
                .checked_mul(10_000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)? as u64;
            
            self.pay_player(payout,bumps.vault)?;
        }
        Ok(())
    }

    pub fn pay_player(&mut self, amount: u64, bump: u8) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };
        let signer_seeds = &[
            b"vault",
            self.house.to_account_info().key.as_ref(),
            &[bump],
        ];
        let signer_seeds = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, amount)
    }
}
