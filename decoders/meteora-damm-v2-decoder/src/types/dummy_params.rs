use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{BorshFeeMarketCapScheduler, BorshFeeRateLimiter, BorshFeeTimeScheduler};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DummyParams {
    pub borsh_fee_time_scheduler_params: BorshFeeTimeScheduler,
    pub borsh_fee_rate_limiter_params: BorshFeeRateLimiter,
    pub borsh_fee_market_cap_scheduler_params: BorshFeeMarketCapScheduler,
}
