use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreatorWithdrawSurplus {
    pub pool: Address,
    pub surplus_amount: u64,
}

impl EvtCreatorWithdrawSurplus {
    pub const DISCRIMINATOR: [u8; 8] = [152, 73, 21, 15, 66, 87, 53, 157];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
