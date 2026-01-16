use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateMeteoraMigrationMetadata {
    pub virtual_pool: Address,
}

impl EvtCreateMeteoraMigrationMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [99, 167, 133, 63, 214, 143, 175, 139];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
