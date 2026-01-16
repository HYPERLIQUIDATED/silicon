use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtSetPoolStatus {
    pub pool: Address,
    pub status: u8,
}

impl EvtSetPoolStatus {
    pub const DISCRIMINATOR: [u8; 8] = [100, 213, 74, 3, 95, 91, 228, 146];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
