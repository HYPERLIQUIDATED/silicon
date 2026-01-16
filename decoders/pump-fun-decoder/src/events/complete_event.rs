use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CompleteEvent {
    pub user: Address,
    pub mint: Address,
    pub bonding_curve: Address,
    pub timestamp: i64,
}

impl CompleteEvent {
    pub const DISCRIMINATOR: [u8; 8] = [95, 114, 97, 156, 212, 46, 152, 8];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
