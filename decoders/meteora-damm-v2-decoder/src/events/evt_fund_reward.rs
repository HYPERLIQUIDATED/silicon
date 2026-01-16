use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtFundReward {
    pub pool: Address,
    pub funder: Address,
    pub mint_reward: Address,
    pub reward_index: u8,
    pub amount: u64,
    pub transfer_fee_excluded_amount_in: u64,
    pub reward_duration_end: u64,
    pub pre_reward_rate: u128,
    pub post_reward_rate: u128,
}

impl EvtFundReward {
    pub const DISCRIMINATOR: [u8; 8] = [104, 233, 237, 122, 199, 191, 121, 85];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
