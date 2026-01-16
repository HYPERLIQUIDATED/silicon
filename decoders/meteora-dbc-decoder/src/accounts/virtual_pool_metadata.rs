use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct VirtualPoolMetadata {
    pub virtual_pool: Address,
    pub padding: [u128; 6],
    pub name: String,
    pub website: String,
    pub logo: String,
}

impl VirtualPoolMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [217, 37, 82, 250, 43, 47, 228, 254];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
