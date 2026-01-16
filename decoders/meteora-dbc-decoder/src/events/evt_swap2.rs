use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{SwapParameters2, SwapResult2};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtSwap2 {
    pub pool: Address,
    pub config: Address,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub swap_parameters: SwapParameters2,
    pub swap_result: SwapResult2,
    pub quote_reserve_amount: u64,
    pub migration_threshold: u64,
    pub current_timestamp: u64,
}

impl EvtSwap2 {
    pub const DISCRIMINATOR: [u8; 8] = [189, 66, 51, 168, 38, 80, 117, 153];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
