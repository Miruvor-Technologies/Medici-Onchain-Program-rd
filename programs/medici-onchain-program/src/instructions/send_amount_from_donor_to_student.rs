use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use crate::{types::Amount, FeesConfigurationAccount};

#[derive(Accounts)]
pub struct SendAmountFromDonorToStudent<'info> {
    pub fees_configuration_account: Account<'info, FeesConfigurationAccount>,
    pub donor: Signer<'info>,
    #[account(mut)]
    pub donor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub student_token_account: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn send_amount_from_donor_to_student_handler(
    ctx: Context<SendAmountFromDonorToStudent>,
    amount: Amount,
) -> Result<()> {

    let fees_percentage = ctx.accounts.fees_configuration_account.fees_configuration.0;

    let fees_amount = amount.0 * (fees_percentage as u64) / 100;
    let student_amount = amount.0 - fees_amount;

    let cpi_accounts = Transfer {
        from: ctx.accounts.donor_token_account.to_account_info(),
        to: ctx.accounts.student_token_account.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    transfer(cpi_ctx, student_amount)?;

    Ok(())
}
