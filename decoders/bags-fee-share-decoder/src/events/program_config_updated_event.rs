use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ProgramConfigUpdatedEvent {
    pub timestamp: i64,
    pub program_config: Address,
    pub new_platform_bps: u16,
    pub old_platform_bps: u16,
    pub pending_admin: Option<Address>,
}

impl ProgramConfigUpdatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [14, 133, 50, 100, 0, 210, 124, 228];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
