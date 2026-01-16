use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LiquidityDistributionParameters {
    pub sqrt_price: u128,
    pub liquidity: u128,
}
