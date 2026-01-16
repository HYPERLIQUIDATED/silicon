use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{PoolFeesStruct, PoolMetrics, RewardInfo};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Pool {
    pub pool_fees: PoolFeesStruct,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub token_a_vault: Address,
    pub token_b_vault: Address,
    pub whitelisted_vault: Address,
    pub partner: Address,
    pub liquidity: u128,
    pub _padding: u128,
    pub protocol_a_fee: u64,
    pub protocol_b_fee: u64,
    pub partner_a_fee: u64,
    pub partner_b_fee: u64,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub activation_type: u8,
    pub pool_status: u8,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub collect_fee_mode: u8,
    pub pool_type: u8,
    pub version: u8,
    pub _padding_0: u8,
    pub fee_a_per_liquidity: [u8; 32],
    pub fee_b_per_liquidity: [u8; 32],
    pub permanent_lock_liquidity: u128,
    pub metrics: PoolMetrics,
    pub creator: Address,
    pub _padding_1: [u64; 6],
    pub reward_infos: [RewardInfo; 2],
}

impl Pool {
    pub const DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
