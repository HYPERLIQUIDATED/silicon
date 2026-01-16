use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtInitializeReward {
    pub pool: Address,
    pub reward_mint: Address,
    pub funder: Address,
    pub creator: Address,
    pub reward_index: u8,
    pub reward_duration: u64,
}

impl EvtInitializeReward {
    pub const DISCRIMINATOR: [u8; 8] = [129, 91, 188, 3, 246, 52, 185, 249];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
