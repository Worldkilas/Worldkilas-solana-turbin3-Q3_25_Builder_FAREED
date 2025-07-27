use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    // token::{ transfer_checked, TransferChecked },
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Listing, Marketplace};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump= marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub seller_mint: InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=seller_mint,
        associated_token::authority= seller
    )]
    pub seller_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=seller,
        space= Listing::INIT_SPACE,
        seeds=[marketplace.key().as_ref(),seller_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        init,
        payer=seller,
        associated_token::mint=seller_mint,
        associated_token::authority= listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            seller_mint.key().as_ref(),
            
        ],
        seeds::program=metadata_program.key(),
        bump
    )]
    pub metadata: Account<'info, MetadataAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub metadata_program: Program<'info, Metadata>
}
