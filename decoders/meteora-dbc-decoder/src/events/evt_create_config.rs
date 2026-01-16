use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{LiquidityDistributionParameters, LockedVestingParams, PoolFeeParameters};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateConfig {
    pub config: Address,
    pub quote_mint: Address,
    pub fee_claimer: Address,
    pub owner: Address,
    pub pool_fees: PoolFeeParameters,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub token_type: u8,
    pub partner_permanent_locked_liquidity_percentage: u8,
    pub partner_liquidity_percentage: u8,
    pub creator_permanent_locked_liquidity_percentage: u8,
    pub creator_liquidity_percentage: u8,
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_amount: u64,
    pub sqrt_start_price: u128,
    pub locked_vesting: LockedVestingParams,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub curve: Vec<LiquidityDistributionParameters>,
}

impl EvtCreateConfig {
    pub const DISCRIMINATOR: [u8; 8] = [131, 207, 180, 174, 180, 73, 165, 54];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
