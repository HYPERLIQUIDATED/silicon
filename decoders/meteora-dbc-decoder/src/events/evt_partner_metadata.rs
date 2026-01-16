use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtPartnerMetadata {
    pub partner_metadata: Address,
    pub fee_claimer: Address,
}

impl EvtPartnerMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [200, 127, 6, 55, 13, 32, 8, 150];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
