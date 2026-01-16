use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BagsFeeSharePartnerClaimEvent {
    pub timestamp: i64,
    pub partner: Address,
    pub partner_config: Address,
    pub claimed: u64,
    pub total_claimed_fees: u128,
}

impl BagsFeeSharePartnerClaimEvent {
    pub const DISCRIMINATOR: [u8; 8] = [180, 40, 86, 176, 242, 25, 220, 77];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
