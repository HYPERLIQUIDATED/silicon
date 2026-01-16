use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigratedPoolMarketCapFeeSchedulerParams {
    pub number_of_period: u16,
    pub sqrt_price_step_bps: u16,
    pub scheduler_expiration_duration: u32,
    pub reduction_factor: u64,
}
