use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtVirtualPoolMetadata {
    pub virtual_pool_metadata: Address,
    pub virtual_pool: Address,
}

impl EvtVirtualPoolMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [188, 18, 72, 76, 195, 91, 38, 74];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
