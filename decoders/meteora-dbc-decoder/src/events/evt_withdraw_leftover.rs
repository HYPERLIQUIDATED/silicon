use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtWithdrawLeftover {
    pub pool: Address,
    pub leftover_receiver: Address,
    pub leftover_amount: u64,
}

impl EvtWithdrawLeftover {
    pub const DISCRIMINATOR: [u8; 8] = [191, 189, 104, 143, 111, 156, 94, 229];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
