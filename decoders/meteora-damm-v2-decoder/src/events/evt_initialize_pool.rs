use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PoolFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtInitializePool {
    pub pool: Address,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub creator: Address,
    pub payer: Address,
    pub alpha_vault: Address,
    pub pool_fees: PoolFeeParameters,
    pub sqrt_min_price: u128,
    pub sqrt_max_price: u128,
    pub activation_type: u8,
    pub collect_fee_mode: u8,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub token_a_flag: u8,
    pub token_b_flag: u8,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub total_amount_a: u64,
    pub total_amount_b: u64,
    pub pool_type: u8,
}

impl EvtInitializePool {
    pub const DISCRIMINATOR: [u8; 8] = [228, 50, 246, 85, 203, 66, 134, 37];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
