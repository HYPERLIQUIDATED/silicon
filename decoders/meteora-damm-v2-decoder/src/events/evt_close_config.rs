use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCloseConfig {
    pub config: Address,
    pub admin: Address,
}

impl EvtCloseConfig {
    pub const DISCRIMINATOR: [u8; 8] = [36, 30, 239, 45, 58, 132, 14, 5];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
