use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SplitPositionInfo {
    pub liquidity: u128,
    pub fee_a: u64,
    pub fee_b: u64,
    pub reward_0: u64,
    pub reward_1: u64,
}
