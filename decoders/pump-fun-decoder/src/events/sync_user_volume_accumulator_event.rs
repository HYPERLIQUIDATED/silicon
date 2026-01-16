use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SyncUserVolumeAccumulatorEvent {
    pub user: Address,
    pub total_claimed_tokens_before: u64,
    pub total_claimed_tokens_after: u64,
    pub timestamp: i64,
}

impl SyncUserVolumeAccumulatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [197, 122, 167, 124, 116, 81, 91, 255];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
