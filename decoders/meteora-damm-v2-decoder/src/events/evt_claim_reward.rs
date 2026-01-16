use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimReward {
    pub pool: Address,
    pub position: Address,
    pub owner: Address,
    pub mint_reward: Address,
    pub reward_index: u8,
    pub total_reward: u64,
}

impl EvtClaimReward {
    pub const DISCRIMINATOR: [u8; 8] = [218, 86, 147, 200, 235, 188, 215, 231];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
