use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Permission {
    pub authority: Address,
    pub padding: [u64; 30],
}

impl Permission {
    pub const DISCRIMINATOR: [u8; 8] = [224, 83, 28, 79, 10, 253, 161, 28];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
