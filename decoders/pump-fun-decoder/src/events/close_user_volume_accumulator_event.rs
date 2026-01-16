use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CloseUserVolumeAccumulatorEvent {
    pub user: Address,
    pub timestamp: i64,
    pub total_unclaimed_tokens: u64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
    pub last_update_timestamp: i64,
}

impl CloseUserVolumeAccumulatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [146, 159, 189, 172, 146, 88, 56, 244];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
