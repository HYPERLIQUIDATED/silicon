use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCloseClaimFeeOperator {
    pub claim_fee_operator: Address,
    pub operator: Address,
}

impl EvtCloseClaimFeeOperator {
    pub const DISCRIMINATOR: [u8; 8] = [111, 39, 37, 55, 110, 216, 194, 23];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
