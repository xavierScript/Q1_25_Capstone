// use anchor_lang::prelude::*;
// use anchor_spl::{associated_token::AssociatedToken, token_interface::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface, TransferChecked, Transfer}};

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, transfer_checked, CloseAccount, Mint, TokenAccount, Token, TransferChecked}};
use anchor_spl::token_interface::TokenInterface;

use crate::state::escrow::Escrow;
use crate::errors::EscrowError;


// create context
#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    pub mint_a: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
    )]
    pub taker_ata_a: Account<'info, TokenAccount>,
    #[account(
        mut,
        close = maker,
        has_one = maker,
        // has_one = taker,
        has_one = mint_a,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
        // vault
    #[account(
    mut,
    associated_token::mint = mint_a,
    associated_token::authority = escrow,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,

   
}


// Transfer tokens from vault to taker
// close vault account

impl<'info> Take<'info> {
    pub fn withdraw_and_close_vault(&mut self) -> Result<()> {
        // Ensure only the registered taker can claim the tokens
        // let taker = self
        //     .escrow
        //     .taker
        //     .ok_or(EscrowError::UnauthorizedTaker)?; // Handle None case
        // require!(taker == self.taker.key(), EscrowError::UnauthorizedTaker);

        // Verify if the vault has enough tokens
        // require!(
        //     self.vault.amount >= self.escrow.receive,
        //     EscrowError::VaultEmpty
        // );

        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"escrow",
            self.maker.key.as_ref(),
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        // WITHDRAW TOKENS
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.clone(), cpi_accounts, &signer_seeds);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_a.decimals)?; // Use escrow.receive, not vault.amount

        // CLOSE VAULT ACCOUNT ONLY IF EMPTY
        if self.vault.amount == 0 {
            let cpi_accounts = CloseAccount {
                account: self.vault.to_account_info(),
                destination: self.maker.to_account_info(),
                authority: self.escrow.to_account_info(),
            };
            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

            close_account(cpi_ctx)?;
        }

        Ok(())
    }
}
