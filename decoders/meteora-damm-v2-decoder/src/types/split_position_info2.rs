use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SplitPositionInfo2 {
    pub unlocked_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub vested_liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
