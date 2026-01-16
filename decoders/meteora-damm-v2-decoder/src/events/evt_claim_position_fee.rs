use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimPositionFee {
    pub pool: Address,
    pub position: Address,
    pub owner: Address,
    pub fee_a_claimed: u64,
    pub fee_b_claimed: u64,
}

impl EvtClaimPositionFee {
    pub const DISCRIMINATOR: [u8; 8] = [198, 182, 183, 52, 97, 12, 49, 56];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
