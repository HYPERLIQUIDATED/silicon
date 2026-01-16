use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectCreatorFeeEvent {
    pub timestamp: i64,
    pub creator: Address,
    pub creator_fee: u64,
}

impl CollectCreatorFeeEvent {
    pub const DISCRIMINATOR: [u8; 8] = [122, 2, 127, 1, 14, 191, 12, 175];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
