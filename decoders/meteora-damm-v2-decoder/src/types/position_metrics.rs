use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PositionMetrics {
    pub total_claimed_a_fee: u64,
    pub total_claimed_b_fee: u64,
}
