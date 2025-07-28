pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("HsRzQ5QEwLrtBVNTBbHQHJAAxvmqD9j2xwAHsNWZawUo");

#[program]
pub mod satoshi_dice_game {
    use super::*;
}
