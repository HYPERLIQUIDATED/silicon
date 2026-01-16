use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetCreatorEvent {
    pub timestamp: i64,
    pub admin_set_creator_authority: Address,
    pub mint: Address,
    pub bonding_curve: Address,
    pub old_creator: Address,
    pub new_creator: Address,
}

impl AdminSetCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [64, 69, 192, 104, 29, 30, 25, 107];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
