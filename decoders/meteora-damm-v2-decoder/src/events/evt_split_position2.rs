use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{SplitAmountInfo, SplitPositionInfo, SplitPositionParameters2};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtSplitPosition2 {
    pub pool: Address,
    pub first_owner: Address,
    pub second_owner: Address,
    pub first_position: Address,
    pub second_position: Address,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo,
    pub first_position_info: SplitPositionInfo,
    pub second_position_info: SplitPositionInfo,
    pub split_position_parameters: SplitPositionParameters2,
}

impl EvtSplitPosition2 {
    pub const DISCRIMINATOR: [u8; 8] = [165, 32, 203, 174, 72, 100, 233, 103];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
