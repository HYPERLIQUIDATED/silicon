use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::Shareholder;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DistributeCreatorFeesEvent {
    pub timestamp: i64,
    pub mint: Address,
    pub bonding_curve: Address,
    pub sharing_config: Address,
    pub admin: Address,
    pub shareholders: Vec<Shareholder>,
    pub distributed: u64,
}

impl DistributeCreatorFeesEvent {
    pub const DISCRIMINATOR: [u8; 8] = [165, 55, 129, 112, 4, 179, 202, 40];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
