use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SwapParameters {
    pub amount_in: u64,
    pub minimum_amount_out: u64,
}
