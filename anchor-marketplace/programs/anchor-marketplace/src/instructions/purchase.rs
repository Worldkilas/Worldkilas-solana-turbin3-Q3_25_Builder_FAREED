use anchor_lang::{ prelude::*, system_program::{ transfer, Transfer } };
use anchor_spl::{
    associated_token::AssociatedToken,
    // token::{ close_account, transfer_checked, CloseAccount, TransferChecked },
    token_interface::{
        close_account,
        transfer_checked,
        CloseAccount,
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
    },
};

use crate::{cpi, Listing, Marketplace};

#[derive(Accounts)]
pub struct Purchase<'info>{
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub lister: SystemAccount<'info>,

    #[account(
        seeds=[b"marketplace",marketplace.name.as_str().as_bytes()],
        bump= marketplace.marketplace_bump
    )]
    pub marketplace: Account<'info,Marketplace>,

    #[account(
        mut,
        seeds=[b"treasury", marketplace.key().as_ref()],
        bump=marketplace.treasury_bump
    )]
    pub treasury: SystemAccount<'info>,

    pub nft_mint: InterfaceAccount<'info,Mint>,

    #[account(
        init_if_needed,
        payer= buyer,
        associated_token::mint= nft_mint,
        associated_token::authority=buyer,
    )]
    pub buyer_nft_ata: InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=nft_mint,
        associated_token::authority=listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close= lister,
        seeds=[marketplace.key().as_ref(),nft_mint.key().as_ref()],
        bump= listing.listing_bump
    )]
    pub listing: Account<'info, Listing>,

    pub token_program: Interface<'info,TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>

}

impl <'info> Purchase<'info> {
    pub fn purchase_nft(&mut self)->Result<()>{
        let marketplace_fees=(self.marketplace.fee/10_000) as u64;
        // transform marketplace_fees
        let marketplace_fees=marketplace_fees*self.listing.price;

        let amount_to_pay_lister=self.listing.price-marketplace_fees;

        self.transfer_nft_to_buyer()?;
        self.take_marketplace_fees(marketplace_fees)?;
        self.pay_lister(amount_to_pay_lister)?;
        self.close_vault()?;
        Ok(())
    }
    
    pub fn transfer_nft_to_buyer(&mut self)->Result<()> {
        let cpi_program= self.token_program.to_account_info();
        let transfer_accounts=TransferChecked{
            from: self.vault.to_account_info(),
            to: self.buyer_nft_ata.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let signer_seeds=&[
            self.marketplace.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.listing.listing_bump]
        ];
        let signer_seeds=&[&signer_seeds[..]];

        let cpi_ctx= CpiContext::new_with_signer(cpi_program, transfer_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault.amount, self.nft_mint.decimals)?;
        Ok(())
    }

    pub fn take_marketplace_fees(&mut self, marketplace_fees:u64)->Result<()> {
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=Transfer{
            from: self.buyer.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let cpi_ctx=CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, marketplace_fees)?;
        Ok(())
    }

    pub fn pay_lister(&mut self, amount: u64)->Result<()>{
        let cpi_program=self.token_program.to_account_info();
        let cpi_accounts=Transfer{
            from: self.buyer.to_account_info(),
            to: self.lister.to_account_info()
        };

        let cpi_ctx= CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount);
        Ok(())
    }

    pub fn close_vault(&mut self)->Result<()> {
        let cpi_progrram= self.token_program.to_account_info();
        let close_accounts=CloseAccount{
            account: self.vault.to_account_info(),
            destination: self.lister.to_account_info(),
            authority: self.listing.to_account_info()
        };

        let signer_seeds=&[
            self.marketplace.to_account_info().key.as_ref(),
            self.nft_mint.to_account_info().key.as_ref(),
            &[self.listing.listing_bump]
        ];
        let signer_seeds=&[&signer_seeds[..]];

        let cpi_ctx=CpiContext::new_with_signer(cpi_progrram, close_accounts, signer_seeds);

        close_account(cpi_ctx)?;
        Ok(())
    }
}