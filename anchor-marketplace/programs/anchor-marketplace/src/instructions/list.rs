use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{MasterEditionAccount, Metadata, MetadataAccount},
    // token::{ transfer_checked, TransferChecked },
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::{Listing, Marketplace};

/// Accounts context for the `list` instruction.  
/// This handles NFT listing by verifying metadata, ensuring it's part of a valid collection,
/// and securely transferring the NFT into a vault owned by the program.
#[derive(Accounts)]
pub struct List<'info> {
    /// The user listing the NFT.
    #[account(mut)]
    pub lister: Signer<'info>,

     /// The marketplace instance this listing belongs to.
    #[account(
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump= marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

      /// The mint address of the NFT being listed.
    pub lister_mint: InterfaceAccount<'info,Mint>,

     /// The mint of the NFT collection this NFT belongs to.
    /// Must match `metadata.collection.key`
    pub collection_mint: InterfaceAccount<'info, Mint>,

     /// The lister’s associated token account holding the NFT.
    /// Must be ATA for `lister` and `lister_mint`.
    #[account(
        mut,
        associated_token::mint=lister_mint,
        associated_token::authority= lister
    )]
    pub lister_ata: InterfaceAccount<'info, TokenAccount>,

    /// PDA where listing data will be stored.
    #[account(
        init,
        payer=lister,
        space= Listing::INIT_SPACE,
        seeds=[marketplace.key().as_ref(),lister_mint.key().as_ref()],
        bump
    )]
    pub listing: Account<'info, Listing>,

    /// The NFT vault account (ATA) owned by the program for this listing.
    /// Will hold the NFT after listing.
    #[account(
        init,
        payer=lister,
        associated_token::mint=lister_mint,
        associated_token::authority= listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

      /// Metadata account associated with the NFT mint.
    /// PDA: seeds = ["metadata", metadata_program_id, lister_mint]
    /// Constraints:
    /// - Must belong to the same collection as `collection_mint`
    /// - Collection must be verified
    #[account(
        seeds=[
            b"metadata",
            metadata_program.key().as_ref(),
            lister_mint.key().as_ref(),
            
        ],
        seeds::program=metadata_program.key(),
        bump,
        constraint= metadata.collection.as_ref().unwrap().key.as_ref()==collection_mint.key().as_ref(),
        constraint= metadata.collection.as_ref().unwrap().verified==true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds=[b"metadata",metadata_program.key().as_ref(),lister_mint.key().as_ref(), b"edition"],
        seeds::program=metadata_program.key(),
        bump,
    )]
    pub master_edition: Account<'info,MasterEditionAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
   
}

impl <'info> List <'info> {
     /// Main handler to list an NFT.
    /// Initializes the listing and transfers the NFT from user to the vault.
    pub fn list(&mut self, price:u64, bumps:&ListBumps)->Result<()> {
        self.create_listing(price, bumps.listing)?;

        self.deposit_nft()?;
        Ok(())
    }

    pub fn create_listing(&mut self,price:u64,bump:u8) ->Result<()>{
        self.listing.set_inner(
            Listing { seller: self.lister.key(), seller_mint: self.lister_mint.key(), listing_bump: bump, price }
        );
        Ok(())
    }

    pub fn deposit_nft(&mut self)->Result<()> {
        let cpi_program= self.token_program.to_account_info();

        let cpi_accounts= TransferChecked{
            from: self.lister_ata.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.lister_mint.to_account_info(),
            authority: self.lister.to_account_info()
        };

        let cpi_ctx= CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, self.lister_ata.amount, self.lister_mint.decimals)?;
        Ok(())
    }
}
