use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtUpdatePoolCreator {
    pub pool: Address,
    pub creator: Address,
    pub new_creator: Address,
}

impl EvtUpdatePoolCreator {
    pub const DISCRIMINATOR: [u8; 8] = [107, 225, 165, 237, 91, 158, 213, 220];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
