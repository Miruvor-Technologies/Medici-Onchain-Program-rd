pub mod constants;
pub mod error;
pub mod instructions;
pub mod seeds;
pub mod state;
pub mod types;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use seeds::*;
pub use state::*;
pub use types::*;

declare_id!("DqswmrLqFwJRudNBBQkv2XUYbTXrVKZsRMoVDrU5wuxK");

#[program]
pub mod medici_onchain_program {
    use super::*;

    pub fn change_fees_configuration(
        ctx: Context<ChangeFeesConfiguration>,
        new_fees_configuration: FeesConfiguration,
    ) -> Result<()> {
        change_fees_configuration_handler(ctx, new_fees_configuration)
    }

    pub fn initialize_fees_configuration_account(
        ctx: Context<InitializeFeesConfigurationAccount>,
    ) -> Result<()> {
        initialize_fees_configuration_account_handler(ctx)
    }

    pub fn send_amount_from_donor_to_student(
        ctx: Context<SendAmountFromDonorToStudent>,
        amount: Amount,
    ) -> Result<()> {
        send_amount_from_donor_to_student_handler(ctx, amount)
    }
}
