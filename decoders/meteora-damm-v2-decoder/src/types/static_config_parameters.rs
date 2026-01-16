use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PoolFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct StaticConfigParameters {
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub vault_config_key: Address,
    pub pool_creator_authority: Address,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
}
