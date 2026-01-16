use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ExtendAccountEvent {
    pub timestamp: i64,
    pub account: Address,
    pub user: Address,
    pub current_size: u64,
    pub new_size: u64,
}

impl ExtendAccountEvent {
    pub const DISCRIMINATOR: [u8; 8] = [97, 97, 215, 144, 93, 146, 22, 124];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
