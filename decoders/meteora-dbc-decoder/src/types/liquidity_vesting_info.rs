use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LiquidityVestingInfo {
    pub is_initialized: u8,
    pub vesting_percentage: u8,
    pub _padding: [u8; 2],
    pub bps_per_period: u16,
    pub number_of_periods: u16,
    pub frequency: u32,
    pub cliff_duration_from_migration_time: u32,
}
