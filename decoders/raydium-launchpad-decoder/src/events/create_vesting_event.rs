use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateVestingEvent {
    pub pool_state: Address,
    pub beneficiary: Address,
    pub share_amount: u64,
}

impl CreateVestingEvent {
    pub const DISCRIMINATOR: [u8; 8] = [150, 152, 11, 179, 52, 210, 191, 125];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
