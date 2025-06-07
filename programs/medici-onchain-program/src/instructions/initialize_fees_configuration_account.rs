use crate::seeds::{FEES_CONFIGURATION_ACCOUNT_SEED, MEDICI_SCHOLARSHIP_PROGRAM_SEED, MEDICI_SEED};
use crate::state::FeesConfigurationAccount;
use crate::types::{CanonicalBump, FeesConfiguration, FeesConfigurationModifyingPubkey};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeFeesConfigurationAccount<'info> {
    #[account(
        init,
        payer = signer,
        space = FeesConfigurationAccount::SIZE,
        seeds = [MEDICI_SEED, MEDICI_SCHOLARSHIP_PROGRAM_SEED, FEES_CONFIGURATION_ACCOUNT_SEED],
        bump,
    )]
    pub fees_configuration_account: Account<'info, FeesConfigurationAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_fees_configuration_account_handler(
    ctx: Context<InitializeFeesConfigurationAccount>,
) -> Result<()> {
    ctx.accounts.fees_configuration_account.fees_configuration = FeesConfiguration::default();
    ctx.accounts
        .fees_configuration_account
        .fees_configuration_modifying_pubkey =
        FeesConfigurationModifyingPubkey(ctx.accounts.signer.key());
    ctx.accounts.fees_configuration_account.bump =
        CanonicalBump(ctx.bumps.fees_configuration_account);

    Ok(())
}
