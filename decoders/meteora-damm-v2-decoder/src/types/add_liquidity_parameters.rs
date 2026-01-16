use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AddLiquidityParameters {
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
}
