pub use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct FeesConfiguration(pub u8);
impl FeesConfiguration {
    pub const SIZE: usize = 1;
    pub fn default() -> Self {
        Self(0)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct CanonicalBump(pub u8);
impl CanonicalBump {
    pub const SIZE: usize = 1;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct FeesConfigurationModifyingPubkey(pub Pubkey);
impl FeesConfigurationModifyingPubkey {
    pub const SIZE: usize = 32;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct Amount(pub u64);
impl Amount {
    pub const SIZE: usize = 8;
}