use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SplitPositionParameters3 {
    pub unlocked_liquidity_numerator: u32,
    pub permanent_locked_liquidity_numerator: u32,
    pub fee_a_numerator: u32,
    pub fee_b_numerator: u32,
    pub reward_0_numerator: u32,
    pub reward_1_numerator: u32,
    pub inner_vesting_liquidity_numerator: u32,
}
