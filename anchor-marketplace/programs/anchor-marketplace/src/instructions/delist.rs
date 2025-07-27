use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    // token::{ close_account, transfer_checked, CloseAccount, TransferChecked },
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{ Listing, Marketplace};
/// Accounts required to delist an NFT from the marketplace.
///
/// This instruction allows a lister to remove a previously listed NFT,
/// closing the listing account and returning custody of the NFT to their ATA.
#[derive(Accounts)]
pub struct Delist<'info> {
    /// The signer who originally listed the NFT.
    /// Must match the authority used in the listing.
    #[account(mut)]
    pub lister: Signer<'info>,

      /// The marketplace where the NFT was listed.
    /// Verified using the PDA derived from the marketplace name.
    #[account(
        seeds=[b"marketplace", marketplace.name.as_str().as_bytes()],
        bump=marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info, Marketplace>,

    pub lister_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=lister_mint,
        associated_token::authority=lister,
    )]
    pub lister_ata: InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        close= lister,
        seeds=[marketplace.key().as_ref(), lister_mint.key().as_ref()],
        bump= listing.listing_bump
    )]
    pub listing: Account<'info, Listing>,

    #[account(
        mut,
        associated_token::mint=lister_mint,
        associated_token::authority=listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,


    pub token_program: Interface<'info, TokenInterface>,
    
}

impl <'info> Delist<'info> {
    pub fn delist(&mut self)->Result<()>{
        self.withdraw_nft()?;
        self.close_vault()?;
        Ok(())
    }

    pub fn withdraw_nft(&mut self)->Result<()>{
        let cpi_program= self.token_program.to_account_info();
        let cpi_accounts=TransferChecked{
            from: self.vault.to_account_info(),
            to: self.lister_ata.to_account_info(),
            mint: self.lister_mint.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let signer_seeds=&[
           self.marketplace.to_account_info().key.as_ref(),
           self.lister_mint.to_account_info().key.as_ref(),
           &[self.listing.listing_bump]
        ];

        let signer_seeds= &[&signer_seeds[..]];

        let cpi_ctx= CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount,self.lister_mint.decimals)?;
        Ok(())
    }

    pub fn close_vault(&mut self)->Result<()> {
        let cpi_program= self.token_program.to_account_info();
        let cpi_accounts=CloseAccount{
            account:self.vault.to_account_info(),
            destination: self.lister.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let signer_seeds=&[
            self.marketplace.to_account_info().key.as_ref(),
            self.lister_mint.to_account_info().key.as_ref(),
            &[self.listing.listing_bump]
        ];
        let signer_seeds=&[&signer_seeds[..]];

        let cpi_ctx= CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        close_account(cpi_ctx)?;
        Ok(())
    }
}
