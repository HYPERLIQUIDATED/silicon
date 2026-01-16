use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PartnerConfigUpdatedEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub partner_config: Address,
    pub partner: Address,
    pub old_bps: u16,
    pub updated_bps: u16,
}

impl PartnerConfigUpdatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [131, 8, 238, 0, 102, 179, 81, 241];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
