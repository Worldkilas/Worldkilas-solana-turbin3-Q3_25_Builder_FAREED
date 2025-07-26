use anchor_lang::prelude::*;

#[error_code]
pub enum StakeError {
    #[msg("Custom Error Message")]
    CustomError,

    #[msg("Max stake amount reached")]
    MaxStakeReached,

    #[msg("Freeze period has not elasped")]
    FreezePeriodNotElasped,

    #[msg("A math overflow occcure")]
    MathOverflow
}
