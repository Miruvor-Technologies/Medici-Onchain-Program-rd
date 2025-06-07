use crate::state::FeesConfigurationAccount;
use crate::types::FeesConfiguration;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ChangeFeesConfiguration<'info> {
    #[account(mut)]
    pub fees_configuration_account: Account<'info, FeesConfigurationAccount>,
    #[account(
        mut,
        address = fees_configuration_account.fees_configuration_modifying_pubkey.0,
    )]
    pub signer: Signer<'info>,
}

pub fn change_fees_configuration_handler(
    ctx: Context<ChangeFeesConfiguration>,
    new_fees_configuration: FeesConfiguration,
) -> Result<()> {
    // Validate that the fee percentage is within valid bounds (0-100)
    require!(
        new_fees_configuration.0 < 100,
        ErrorCode::FeePercentageTooHigh
    );

    // Update the fees configuration
    ctx.accounts.fees_configuration_account.fees_configuration = new_fees_configuration;

    // Validate that the state is consistent
    validate_state_consistency(&ctx)?;

    Ok(())
}

fn validate_state_consistency(ctx: &Context<ChangeFeesConfiguration>) -> Result<()> {
    let account = &ctx.accounts.fees_configuration_account;

    // Verify the signer matches the authorized modifying pubkey
    require!(
        ctx.accounts.signer.key() == account.fees_configuration_modifying_pubkey.0,
        ErrorCode::UnauthorizedSigner
    );

    // Verify the fees configuration is still within bounds after assignment
    require!(
        account.fees_configuration.0 < 100,
        ErrorCode::StateInconsistentFeeConfiguration
    );

    Ok(())
}

#[error_code]
pub enum ErrorCode {
    #[msg("Fee percentage cannot be greater than 100")]
    FeePercentageTooHigh,
    #[msg("Fee percentage cannot be negative")]
    FeePercentageNegative,
    #[msg("Unauthorized signer")]
    UnauthorizedSigner,
    #[msg("State inconsistent with fees configuration")]
    StateInconsistentFeeConfiguration,
}
