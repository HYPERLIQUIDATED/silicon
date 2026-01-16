use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PoolFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateConfig {
    pub pool_fees: PoolFeeParameters,
    pub vault_config_key: Address,
    pub pool_creator_authority: Address,
    pub activation_type: u8,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub collect_fee_mode: u8,
    pub index: u64,
    pub config: Address,
}

impl EvtCreateConfig {
    pub const DISCRIMINATOR: [u8; 8] = [131, 207, 180, 174, 180, 73, 165, 54];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
