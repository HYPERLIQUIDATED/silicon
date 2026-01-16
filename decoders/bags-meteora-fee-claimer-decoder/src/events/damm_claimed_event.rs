use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DAMMClaimedEvent {
    pub claimer: Address,
    pub damm_total_fees: u64,
    pub claimer_share: u64,
    pub total_claimer_fees: u64,
    pub partner_share: u64,
    pub total_partner_fees: u64,
}

impl DAMMClaimedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [218, 142, 187, 210, 111, 192, 166, 237];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
