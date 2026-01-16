use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimProtocolFee {
    pub pool: Address,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

impl EvtClaimProtocolFee {
    pub const DISCRIMINATOR: [u8; 8] = [186, 244, 75, 251, 188, 13, 25, 33];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
