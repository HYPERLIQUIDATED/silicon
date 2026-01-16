use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::InnerVesting;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Vesting {
    pub position: Address,
    pub inner_vesting: InnerVesting,
    pub padding2: [u128; 4],
}

impl Vesting {
    pub const DISCRIMINATOR: [u8; 8] = [100, 149, 66, 138, 95, 200, 128, 241];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
