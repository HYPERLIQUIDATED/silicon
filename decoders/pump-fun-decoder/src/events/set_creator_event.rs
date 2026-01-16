use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetCreatorEvent {
    pub timestamp: i64,
    pub mint: Address,
    pub bonding_curve: Address,
    pub creator: Address,
}

impl SetCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [237, 52, 123, 37, 245, 251, 72, 210];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
