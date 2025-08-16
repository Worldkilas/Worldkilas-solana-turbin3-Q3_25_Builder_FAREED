// Import MarketplaceError from its module

#[macro_export]
macro_rules! require_campaign_not_finalized {
    ($is_finalized: expr) => {
        require!(
            !$is_finalized,
            $crate::error::MarketplaceError::CampaignFinalized
        );
    };
}

#[macro_export]
macro_rules! require_campaign_active {
    ($now: expr, $campaign: expr) => {
        require!(
            $now >= $campaign.start_timestamp && $now <= $campaign.end_timestamp,
            $crate::error::MarketplaceError::CampaignNotActive
        );
    };
}

#[macro_export]
macro_rules! require_campaign_finalized_and_successful {
    ($campaign: expr) => {
        require!(
            $campaign.is_finalized && $campaign.is_successful,
            $crate::error::MarketplaceError::CampaignNotSuccessful
        )
    };
}
