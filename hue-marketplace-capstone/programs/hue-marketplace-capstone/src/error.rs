use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid timestamps")]
    InvalidTimestamps,
    #[msg("Campaign is finalized")]
    CampaignFinalized,
    #[msg("Campaign not active")]
    CampaignNotActive,
    #[msg("Invalid supporter owner")]
    InvalidSupporterOwner,
    #[msg("Overflow")]
    Overflow,
    #[msg("Invalid price")]
    InvalidPrice,
    #[msg("Already finalized")]
    AlreadyFinalized,
    #[msg("Unit ordered exceeds allowed units per supporter or is zero")]
    InvalidUnitsOrdered,
    #[msg("Too early to finalize")]
    TooEarlyToFinalize,
    #[msg("Campaign not successful")]
    CampaignNotSuccessful,
    #[msg("Cannot refund from an already successful campaign")]
    CampaignSuccessful,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Underflow")]
    Underflow,
    #[msg("Already withdrawn")]
    AlreadyWithdrawn,
    #[msg("Already refunded")]
    AlreadyRefunded,
    #[msg("Already minted")]
    AlreadyMinted,
    #[msg("InvalidFeePoints")]
    InvalidFeePoints,
    #[msg("Collection already full")]
    CollectionFull,
}
