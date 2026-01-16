use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateClaimFeeOperator {
    pub operator: Address,
}

impl EvtCreateClaimFeeOperator {
    pub const DISCRIMINATOR: [u8; 8] = [21, 6, 153, 120, 68, 116, 28, 177];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
