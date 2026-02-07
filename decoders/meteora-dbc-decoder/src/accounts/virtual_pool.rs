use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{PoolMetrics, VolatilityTracker};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct VirtualPool {
    pub volatility_tracker: VolatilityTracker,
    pub config: Address,
    pub creator: Address,
    pub base_mint: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub base_reserve: u64,
    pub quote_reserve: u64,
    pub protocol_base_fee: u64,
    pub protocol_quote_fee: u64,
    pub partner_base_fee: u64,
    pub partner_quote_fee: u64,
    pub sqrt_price: u128,
    pub activation_point: u64,
    pub pool_type: u8,
    pub is_migrated: u8,
    pub is_partner_withdraw_surplus: u8,
    pub is_protocol_withdraw_surplus: u8,
    pub migration_progress: u8,
    pub is_withdraw_leftover: u8,
    pub is_creator_withdraw_surplus: u8,
    pub migration_fee_withdraw_status: u8,
    pub metrics: PoolMetrics,
    pub finish_curve_timestamp: u64,
    pub creator_base_fee: u64,
    pub creator_quote_fee: u64,
    pub legacy_creation_fee_bits: u8,
    pub creation_fee_bits: u8,
    pub has_swap: u8,
    pub _padding_0: [u8; 5],
    pub protocol_liquidity_migration_fee_bps: u16,
    pub _padding_1: [u8; 6],
    pub protocol_migration_base_fee_amount: u64,
    pub protocol_migration_quote_fee_amount: u64,
    pub _padding_2: [u64; 3],
}

impl VirtualPool {
    pub const DISCRIMINATOR: [u8; 8] = [213, 224, 5, 209, 98, 69, 119, 92];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
