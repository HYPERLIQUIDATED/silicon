use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{PoolStatus, TradeDirection};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TradeEvent {
    pub pool_state: Address,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base_before: u64,
    pub real_quote_before: u64,
    pub real_base_after: u64,
    pub real_quote_after: u64,
    pub amount_in: u64,
    pub amount_out: u64,
    pub protocol_fee: u64,
    pub platform_fee: u64,
    pub creator_fee: u64,
    pub share_fee: u64,
    pub trade_direction: TradeDirection,
    pub pool_status: PoolStatus,
    pub exact_in: bool,
}

impl TradeEvent {
    pub const DISCRIMINATOR: [u8; 8] = [189, 219, 127, 211, 78, 230, 97, 238];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
