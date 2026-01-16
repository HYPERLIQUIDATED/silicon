use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PartnerConfigCreatedEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub partner_config: Address,
    pub partner: Address,
    pub bps: u16,
}

impl PartnerConfigCreatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [110, 5, 186, 72, 118, 105, 115, 126];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
