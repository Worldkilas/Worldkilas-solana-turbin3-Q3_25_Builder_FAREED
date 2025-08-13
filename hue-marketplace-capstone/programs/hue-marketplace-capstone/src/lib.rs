pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
pub mod helper_macros;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use helper_macros::*;

declare_id!("4V8C51KJ45atM2PBmYp91N2tQLHbzMThvMHvvzFKzAQT");

#[program]
pub mod hue_marketplace_capstone {

    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, fee_bps: u16) -> Result<()> {
        ctx.accounts.init_marketplace(fee_bps, &ctx.bumps)
    }
}
