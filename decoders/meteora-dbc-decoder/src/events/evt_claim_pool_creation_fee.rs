use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimPoolCreationFee {
    pub pool: Address,
    pub receiver: Address,
    pub creation_fee: u64,
}

impl EvtClaimPoolCreationFee {
    pub const DISCRIMINATOR: [u8; 8] = [149, 111, 149, 44, 136, 64, 175, 62];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
