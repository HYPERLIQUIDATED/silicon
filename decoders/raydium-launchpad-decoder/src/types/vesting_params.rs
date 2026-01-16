use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct VestingParams {
    pub total_locked_amount: u64,
    pub cliff_period: u64,
    pub unlock_period: u64,
}
