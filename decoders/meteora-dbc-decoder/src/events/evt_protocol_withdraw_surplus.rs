use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtProtocolWithdrawSurplus {
    pub pool: Address,
    pub surplus_amount: u64,
}

impl EvtProtocolWithdrawSurplus {
    pub const DISCRIMINATOR: [u8; 8] = [109, 111, 28, 221, 134, 195, 230, 203];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
