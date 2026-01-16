use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateAdminEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub new_admin: Address,
}

impl UpdateAdminEvent {
    pub const DISCRIMINATOR: [u8; 8] = [225, 152, 171, 87, 246, 63, 66, 234];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
