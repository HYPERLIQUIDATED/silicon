use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateGlobalAuthorityEvent {
    pub global: Address,
    pub authority: Address,
    pub new_authority: Address,
    pub timestamp: i64,
}

impl UpdateGlobalAuthorityEvent {
    pub const DISCRIMINATOR: [u8; 8] = [182, 195, 137, 42, 35, 206, 207, 247];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
