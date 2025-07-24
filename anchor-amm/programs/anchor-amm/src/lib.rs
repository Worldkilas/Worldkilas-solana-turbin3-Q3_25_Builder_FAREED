pub mod constants;
pub mod error;
pub mod helpers;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6HPwmQCHtUC5kgwo4KvMRAWisuZJi9XZwEJCn7q3vkte");

#[program]
pub mod anchor_amm {
    
    use super::*;
   
    
    /// Arguments used to initialize a new liquidity pool.
    /// - `seed`: Unique seed used to derive the config PDA.
    /// - `fee`: Fee in basis points (e.g. 30 = 0.3%) applied on swaps.
    /// - `authority`: Optional admin override. If `None`, the initializer becomes authority.
    ///
    /// This struct is passed to the `initialize` instruction to configure the pool.
    #[derive(AnchorSerialize,AnchorDeserialize)]
    pub struct InitArgs{
        pub seed: u64,
        pub fee: u16,
        pub authority: Option<Pubkey>

    }

    pub fn initialize(ctx: Context<Initialize>,args: InitArgs)-> Result<()>{
        ctx.accounts.initialize(args.seed,args.fee,args.authority, &ctx.bumps)
        
    }

    pub fn deposit(ctx: Context<Deposit>,lp_claim_amount:u64,max_x:u64,max_y:u64)->Result<()>{
        ctx.accounts.deposit(lp_claim_amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_burn_amount:u64, min_x:u64,min_y:u64)->Result<()>{
        ctx.accounts.withdraw(lp_burn_amount, min_x, min_y)
    }

    pub fn swap(ctx: Context<Swap>, is_token_x:bool, amount_in:u64,min_amount_out:u64)-> Result<()>{
        ctx.accounts.swap_tokens(is_token_x, amount_in, min_amount_out)
    }
}
