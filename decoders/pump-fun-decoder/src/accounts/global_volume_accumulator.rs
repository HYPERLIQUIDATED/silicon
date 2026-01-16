use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct GlobalVolumeAccumulator {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub mint: Address,
    pub total_token_supply: [u64; 30],
    pub sol_volumes: [u64; 30],
}

impl GlobalVolumeAccumulator {
    pub const DISCRIMINATOR: [u8; 8] = [202, 42, 246, 43, 142, 190, 30, 255];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
