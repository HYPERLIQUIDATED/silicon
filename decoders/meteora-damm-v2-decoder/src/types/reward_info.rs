use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RewardInfo {
    pub initialized: u8,
    pub reward_token_flag: u8,
    pub _padding_0: [u8; 6],
    pub _padding_1: [u8; 8],
    pub mint: Address,
    pub vault: Address,
    pub funder: Address,
    pub reward_duration: u64,
    pub reward_duration_end: u64,
    pub reward_rate: u128,
    pub reward_per_token_stored: [u8; 32],
    pub last_update_time: u64,
    pub cumulative_seconds_with_empty_liquidity_reward: u64,
}
