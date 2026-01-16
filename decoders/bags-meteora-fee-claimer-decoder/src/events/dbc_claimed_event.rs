use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DBCClaimedEvent {
    pub claimer: Address,
    pub dbc_total_fees: u64,
    pub claimer_share: u64,
    pub total_claimer_fees: u64,
    pub partner_share: u64,
    pub total_partner_fees: u64,
}

impl DBCClaimedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [144, 46, 37, 27, 10, 171, 177, 31];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
