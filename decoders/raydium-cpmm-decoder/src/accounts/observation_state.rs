use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::Observation;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ObservationState {
    pub initialized: bool,
    pub observation_index: u16,
    pub pool_id: Address,
    pub observations: [Observation; 100],
    pub padding: [u64; 4],
}

impl ObservationState {
    pub const DISCRIMINATOR: [u8; 8] = [122, 174, 197, 53, 129, 9, 165, 132];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
