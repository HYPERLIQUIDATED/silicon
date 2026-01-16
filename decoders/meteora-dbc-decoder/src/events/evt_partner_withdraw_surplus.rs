use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtPartnerWithdrawSurplus {
    pub pool: Address,
    pub surplus_amount: u64,
}

impl EvtPartnerWithdrawSurplus {
    pub const DISCRIMINATOR: [u8; 8] = [195, 56, 152, 9, 232, 72, 35, 22];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
