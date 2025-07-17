use crate::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

/*
   taker_ata_for_b----> maker_ata_for_b
   vault----> taker_ata_for_a

*/

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program=token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program=token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    /// ATA for taker to recieve A tokens
    /// It is also worth noting that the in the case of recieiving tokens,
    /// it may be such that the receiver has no ATA for the token he wants to recieve
    /// So the init_if_needed constraint will handle this case
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_a,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_for_token_a: InterfaceAccount<'info, TokenAccount>,

    /// ATA for taker to send B tokens
    #[account(
         mut,
        associated_token::mint=mint_b,
        associated_token::authority=taker,
        associated_token::token_program=token_program
    )]
    pub taker_ata_for_token_b: InterfaceAccount<'info, TokenAccount>,

    /// ATA for maker to recive B tokens
    /// The maker may also have to init an ATA for token B
    #[account(
        init_if_needed,
        payer=taker,
        associated_token::mint=mint_b,
        associated_token::authority=maker,
        associated_token::token_program=token_program,

    )]
    pub maker_ata_for_token_b: InterfaceAccount<'info, TokenAccount>,
    /// The vault where token A was locked up
    /// Token A was sent by the maker
    #[account(
        mut,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close=maker,
        has_one=maker,
        has_one=mint_a,
        has_one=mint_b,
        seeds=[b"escrow",maker.key().as_ref(), escrow.discriminator.to_le_bytes().as_ref()],
        bump=escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn deposit_to_maker(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_for_token_b.to_account_info(),
            to: self.maker_ata_for_token_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer_checked(cpi_ctx, self.escrow.receive_amount, self.mint_b.decimals)?;
        Ok(())
    }

    pub fn withdraw_from_vault_and_close(&mut self) -> Result<()> {
        let withdraw_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_for_token_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        let signer_seeds = [
            b"escrow",
            self.maker.to_account_info().key.as_ref(),
            &self.escrow.discriminator.to_le_bytes()[..],
            &[self.escrow.bump],
        ];
        // Transform the signer seeds into a slice of slices
        // This is necessary because `CpiContext::new_with_signer` expects a slice of slices
        // The first slice is the seeds, and the second slice is the bump
        let signer_seeds = &[&signer_seeds[..]];

        let withdraw_cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            withdraw_accounts,
            signer_seeds,
        );

        let close_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        transfer_checked(withdraw_cpi_ctx, self.vault.amount, self.mint_a.decimals)?;

        close_account(close_ctx)?;

        Ok(())
    }
}
