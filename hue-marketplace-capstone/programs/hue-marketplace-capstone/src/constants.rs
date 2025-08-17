use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const BASIS_FEE_POINTS: u64 = 10_000;

#[constant]
pub const ONCHAIN_METAPLEX_ORACLE_PLUGIN: Pubkey =
    Pubkey::from_str_const("AwPRxL5f6GDVajyE1bBcfSWdQT58nWMoS36A1uFtpCZY");

#[constant]
pub const DROP_CAMPAIGN_BINARY_STRING: &[u8] = b"drop_campaign";

#[constant]
pub const CONFIG_BINARY_STRING: &[u8] = b"config";

#[constant]
pub const SUPPORTER_BINARY_STRING: &[u8] = b"supporter";

#[constant]
pub const TREASURY_BINARY_STRING: &[u8] = b"treasury";
