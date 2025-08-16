use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

#[constant]
pub const BASIS_FEE_POINTS: u64 = 10_000;

#[constant]
pub const ONCHAIN_METAPLEX_ORACLE_PLUGIN: Pubkey =
    Pubkey::from_str_const("GxaWxaQVeaNeFHehFQEDeKR65MnT6Nup81AGwh2EEnuq");
