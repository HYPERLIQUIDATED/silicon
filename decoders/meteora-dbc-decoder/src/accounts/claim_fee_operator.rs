use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimFeeOperator {
    pub operator: Address,
    pub _padding: [u8; 128],
}

impl ClaimFeeOperator {
    pub const DISCRIMINATOR: [u8; 8] = [166, 48, 134, 86, 34, 200, 188, 150];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
