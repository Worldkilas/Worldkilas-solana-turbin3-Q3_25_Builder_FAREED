pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HGYmo6dFcDhvzyaGHwLYUkSWqZZp8BoEHDuJnMobMWkP");

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize_marketplace(ctx: Context<InitializeMarketplace>, name: String, fee:u16)->Result<()>{
        ctx.accounts.init_marketplace(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price:u64)->Result<()> {
        ctx.accounts.list(price, &ctx.bumps)
        
    }
}
