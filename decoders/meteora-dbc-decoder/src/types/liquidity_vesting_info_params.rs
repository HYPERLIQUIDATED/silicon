use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LiquidityVestingInfoParams {
    pub vesting_percentage: u8,
    pub bps_per_period: u16,
    pub number_of_periods: u16,
    pub cliff_duration_from_migration_time: u32,
    pub frequency: u32,
}
