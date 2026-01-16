use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{SplitAmountInfo2, SplitPositionInfo2, SplitPositionParameters3};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtSplitPosition3 {
    pub pool: Address,
    pub first_owner: Address,
    pub second_owner: Address,
    pub first_position: Address,
    pub second_position: Address,
    pub current_sqrt_price: u128,
    pub amount_splits: SplitAmountInfo2,
    pub first_position_info: SplitPositionInfo2,
    pub second_position_info: SplitPositionInfo2,
    pub split_position_parameters: SplitPositionParameters3,
}

impl EvtSplitPosition3 {
    pub const DISCRIMINATOR: [u8; 8] = [232, 117, 190, 218, 85, 162, 207, 78];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
