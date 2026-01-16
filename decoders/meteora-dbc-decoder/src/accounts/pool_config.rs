use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{
    LiquidityDistributionConfig, LiquidityVestingInfo, LockedVestingConfig, PoolFeesConfig,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolConfig {
    pub quote_mint: Address,
    pub fee_claimer: Address,
    pub leftover_receiver: Address,
    pub pool_fees: PoolFeesConfig,
    pub partner_liquidity_vesting_info: LiquidityVestingInfo,
    pub creator_liquidity_vesting_info: LiquidityVestingInfo,
    pub padding_0: [u8; 14],
    pub padding_1: u16,
    pub collect_fee_mode: u8,
    pub migration_option: u8,
    pub activation_type: u8,
    pub token_decimal: u8,
    pub version: u8,
    pub token_type: u8,
    pub quote_token_flag: u8,
    pub partner_permanent_locked_liquidity_percentage: u8,
    pub partner_liquidity_percentage: u8,
    pub creator_permanent_locked_liquidity_percentage: u8,
    pub creator_liquidity_percentage: u8,
    pub migration_fee_option: u8,
    pub fixed_token_supply_flag: u8,
    pub creator_trading_fee_percentage: u8,
    pub token_update_authority: u8,
    pub migration_fee_percentage: u8,
    pub creator_migration_fee_percentage: u8,
    pub padding_2: [u8; 7],
    pub swap_base_amount: u64,
    pub migration_quote_threshold: u64,
    pub migration_base_threshold: u64,
    pub migration_sqrt_price: u128,
    pub locked_vesting_config: LockedVestingConfig,
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
    pub migrated_collect_fee_mode: u8,
    pub migrated_dynamic_fee: u8,
    pub migrated_pool_fee_bps: u16,
    pub migrated_pool_base_fee_mode: u8,
    pub enable_first_swap_with_min_fee: u8,
    pub _padding_1: [u8; 2],
    pub pool_creation_fee: u64,
    pub migrated_pool_base_fee_bytes: [u8; 16],
    pub sqrt_start_price: u128,
    pub curve: [LiquidityDistributionConfig; 20],
}

impl PoolConfig {
    pub const DISCRIMINATOR: [u8; 8] = [26, 108, 14, 123, 116, 230, 129, 43];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
