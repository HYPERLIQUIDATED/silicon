use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UserRewardInfo {
    pub reward_per_token_checkpoint: [u8; 32],
    pub reward_pendings: u64,
    pub total_claimed_rewards: u64,
}
