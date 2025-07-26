pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6H4PrVFScRUdg1Ga9JSjoaBkZ6arfAZu6LMNtH8rbsLJ");

#[program]
pub mod anchor_nft_staking {
    use super::*;

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }
}
