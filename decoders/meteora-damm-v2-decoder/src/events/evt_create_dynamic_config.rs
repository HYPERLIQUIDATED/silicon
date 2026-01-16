use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateDynamicConfig {
    pub config: Address,
    pub pool_creator_authority: Address,
    pub index: u64,
}

impl EvtCreateDynamicConfig {
    pub const DISCRIMINATOR: [u8; 8] = [231, 197, 13, 164, 248, 213, 133, 152];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
