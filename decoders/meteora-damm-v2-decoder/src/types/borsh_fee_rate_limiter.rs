use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BorshFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 3],
}
