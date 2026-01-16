use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimVestedEvent {
    pub pool_state: Address,
    pub beneficiary: Address,
    pub claim_amount: u64,
}

impl ClaimVestedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [21, 194, 114, 87, 120, 211, 226, 32];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
