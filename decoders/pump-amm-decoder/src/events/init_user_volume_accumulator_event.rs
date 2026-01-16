use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitUserVolumeAccumulatorEvent {
    pub payer: Address,
    pub user: Address,
    pub timestamp: i64,
}

impl InitUserVolumeAccumulatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [134, 36, 13, 72, 232, 101, 130, 216];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
