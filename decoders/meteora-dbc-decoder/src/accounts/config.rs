use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PoolFees;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Config {
    pub pool_fees: PoolFees,
    pub activation_duration: u64,
    pub vault_config_key: Address,
    pub pool_creator_authority: Address,
    pub activation_type: u8,
    pub partner_fee_numerator: u64,
    pub padding: [u8; 219],
}

impl Config {
    pub const DISCRIMINATOR: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
