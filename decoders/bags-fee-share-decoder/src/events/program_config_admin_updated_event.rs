use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ProgramConfigAdminUpdatedEvent {
    pub timestamp: i64,
    pub program_config: Address,
    pub old_admin: Address,
    pub new_admin: Address,
}

impl ProgramConfigAdminUpdatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [19, 215, 161, 37, 149, 193, 142, 93];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
