use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetMetaplexCreatorEvent {
    pub timestamp: i64,
    pub mint: Address,
    pub bonding_curve: Address,
    pub metadata: Address,
    pub creator: Address,
}

impl SetMetaplexCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [142, 203, 6, 32, 127, 105, 191, 162];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
