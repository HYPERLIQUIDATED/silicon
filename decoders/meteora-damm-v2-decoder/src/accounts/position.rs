use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{InnerVesting, PositionMetrics, UserRewardInfo};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Position {
    pub pool: Address,
    pub nft_mint: Address,
    pub fee_a_per_token_checkpoint: [u8; 32],
    pub fee_b_per_token_checkpoint: [u8; 32],
    pub fee_a_pending: u64,
    pub fee_b_pending: u64,
    pub unlocked_liquidity: u128,
    pub vested_liquidity: u128,
    pub permanent_locked_liquidity: u128,
    pub metrics: PositionMetrics,
    pub reward_infos: [UserRewardInfo; 2],
    pub inner_vesting: InnerVesting,
    pub padding: u128,
}

impl Position {
    pub const DISCRIMINATOR: [u8; 8] = [170, 188, 143, 228, 122, 64, 247, 208];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
