use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::UpdatePoolFeesParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtUpdatePoolFees {
    pub pool: Address,
    pub operator: Address,
    pub params: UpdatePoolFeesParameters,
}

impl EvtUpdatePoolFees {
    pub const DISCRIMINATOR: [u8; 8] = [76, 165, 246, 102, 102, 217, 156, 44];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
