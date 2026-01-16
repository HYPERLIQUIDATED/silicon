use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimPartnerFee {
    pub pool: Address,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

impl EvtClaimPartnerFee {
    pub const DISCRIMINATOR: [u8; 8] = [118, 99, 77, 10, 226, 1, 1, 87];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
