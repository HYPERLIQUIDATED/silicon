use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ProgramConfigInitializedEvent {
    pub timestamp: i64,
    pub program_config: Address,
    pub platform_vault: Address,
    pub platform_bps: u16,
    pub admin: Address,
}

impl ProgramConfigInitializedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [210, 169, 156, 90, 13, 171, 254, 140];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
