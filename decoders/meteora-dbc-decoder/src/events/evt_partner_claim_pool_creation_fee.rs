use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtPartnerClaimPoolCreationFee {
    pub pool: Address,
    pub partner: Address,
    pub creation_fee: u64,
    pub fee_receiver: Address,
}

impl EvtPartnerClaimPoolCreationFee {
    pub const DISCRIMINATOR: [u8; 8] = [174, 223, 44, 150, 145, 98, 89, 195];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
