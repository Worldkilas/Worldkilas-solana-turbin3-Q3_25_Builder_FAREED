use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::{
    prelude::*,
    solana_program::{self, ed25519_program, sysvar::instructions::load_instruction_at_checked},
    system_program::{transfer, Transfer},
};
use solana_program::hash::hash;

use crate::{error::DiceError, Bet};
/// House edge in basis points (1.5%)
pub const HOUSE_EDGE: u16 = 150;

/// This instruction resolves a bet placed by a user and determines the payout based on
/// a pseudo-random dice roll derived from an Ed25519 signature provided by the house.
#[derive(Accounts)]
pub struct ResolveBet<'info> {
    /// The house authority who signs the Ed25519 message.
    #[account(mut)]
    pub house: Signer<'info>,
    ///CHECK: we are only receiving lamports + verified through bet.has_one
    pub player: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds=[b"vault",house.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    /// The bet account associated with the player.
    /// PDA derived as: seeds = [b"bet", vault.key(), bet.seed.to_be_bytes()]
    /// Closes to the player after resolution.
    #[account(
        mut,
        close=player,
        has_one=player,
        seeds=[b"bet", vault.key().as_ref(), bet.seed.to_be_bytes().as_ref()],
        bump=bet.bump
    )]
    pub bet: Account<'info, Bet>,

    /// Required for Ed25519 signature verification.
    #[account(
        address=solana_program::sysvar::instructions::ID
    )]
    pub instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    /// Verifies the Ed25519 signature submitted by the house to ensure that the randomness
    /// is cryptographically verifiable and derived from the exact bet state.
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

    /// Resolves the bet by deriving a random roll from the hashed signature and computing payout.
    /// The roll is between 1 and 100 (inclusive). If the rolled value is less than the
    /// player's chosen threshold, the player wins and receives a payout.
    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        // Step 1: Hash the signature to derive randomness
        let hash = hash(sig).to_bytes();

        // Step 2: Split into two 128-bit chunks for more entropy
        let mut hash_16: [u8; 16] = [0; 16];

        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);

        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        // Step 3: Combine and normalize to range [1, 100]
        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        // Step 4: Determine outcome and payout
        if self.bet.roll > roll {
            // Player wins. Calculate payout.
            //
            // Human-readable formula using basis points:
            //
            // payout = (bet_amount * (10_000 - house_edge)) / ((player_roll - 1) * 100)
            //
            // Where:
            // - `house_edge` is in basis points (bps), e.g. 150 bps = 1.5%
            // - 10_000 bps = 100%
            // - We multiply by 10_000 and divide by 100 to preserve precision and simulate percentages.
            //
            // Example:
            // - bet_amount = 10_000 (in lamports or any unit)
            // - player_roll = 50
            // - house_edge = 150 (i.e. 1.5%)
            //
            // payout = (10_000 * (10_000 - 150)) / ((50 - 1) * 100)
            //        = (10_000 * 9_850) / (49 * 100)
            //        = 98_500_000 / 4_900
            //        ≈ 20102 lamports

            let payout = (self.bet.bet_amount as u128)
                .checked_mul(10_000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)? as u64;

            self.pay_player(payout, bumps.vault)?;
        }
        Ok(())
    }

    pub fn pay_player(&mut self, amount: u64, bump: u8) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.player.to_account_info(),
        };
        let signer_seeds = &[b"vault", self.house.to_account_info().key.as_ref(), &[bump]];
        let signer_seeds = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );
        transfer(cpi_ctx, amount)
    }
}
