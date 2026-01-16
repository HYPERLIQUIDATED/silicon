use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PoolFeesConfig;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Config {
    pub vault_config_key: Address,
    pub pool_creator_authority: Address,
    pub pool_fees: PoolFeesConfig,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub config_type: u8,
    pub _padding_0: [u8; 5],
    pub index: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub _padding_1: [u64; 10],
}

impl Config {
    pub const DISCRIMINATOR: [u8; 8] = [155, 12, 170, 224, 30, 250, 204, 130];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
