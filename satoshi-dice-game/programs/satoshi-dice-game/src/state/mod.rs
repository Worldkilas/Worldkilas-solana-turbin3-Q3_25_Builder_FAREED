use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct Bet {
    pub player: Pubkey,
    pub bet_amount: u64,
    pub seed: u64,
    pub slot: u64,
    pub roll: u8,
    pub bump: u8,
}

impl Bet {
    pub fn to_slice(&self) -> Vec<u8> {
        let mut slice = self.player.to_bytes().to_vec();
        slice.extend_from_slice(self.bet_amount.to_le_bytes().as_ref());
        slice.extend_from_slice(self.seed.to_le_bytes().as_ref());
        slice.extend_from_slice(self.slot.to_le_bytes().as_ref());
        slice.extend_from_slice(&[self.roll, self.bump]);

        slice
    }
}
