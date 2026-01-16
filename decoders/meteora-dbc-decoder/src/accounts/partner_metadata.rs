use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PartnerMetadata {
    pub fee_claimer: Address,
    pub padding: [u128; 6],
    pub name: String,
    pub website: String,
    pub logo: String,
}

impl PartnerMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [68, 68, 130, 19, 16, 209, 98, 156];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
