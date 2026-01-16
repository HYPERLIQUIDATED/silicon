use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializePoolParameters {
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub activation_point: Option<u64>,
}
