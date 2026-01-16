use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtUpdateRewardDuration {
    pub pool: Address,
    pub reward_index: u8,
    pub old_reward_duration: u64,
    pub new_reward_duration: u64,
}

impl EvtUpdateRewardDuration {
    pub const DISCRIMINATOR: [u8; 8] = [149, 135, 65, 231, 129, 153, 65, 57];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
