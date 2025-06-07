use anchor_lang::prelude::*;

use crate::constants::DISCRIMINATOR_SIZE;
use crate::types::{CanonicalBump, FeesConfiguration, FeesConfigurationModifyingPubkey};

#[account]
pub struct FeesConfigurationAccount {
    pub fees_configuration: FeesConfiguration,
    pub fees_configuration_modifying_pubkey: FeesConfigurationModifyingPubkey,
    pub bump: CanonicalBump,
}

impl FeesConfigurationAccount {
    pub const SIZE: usize = DISCRIMINATOR_SIZE
        + FeesConfiguration::SIZE
        + FeesConfigurationModifyingPubkey::SIZE
        + CanonicalBump::SIZE;
}
