use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{SwapParameters, SwapResult};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtSwap {
    pub pool: Address,
    pub config: Address,
    pub trade_direction: u8,
    pub has_referral: bool,
    pub params: SwapParameters,
    pub swap_result: SwapResult,
    pub amount_in: u64,
    pub current_timestamp: u64,
}

impl EvtSwap {
    pub const DISCRIMINATOR: [u8; 8] = [27, 60, 21, 213, 138, 170, 187, 147];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
