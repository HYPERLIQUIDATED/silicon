use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtInitializePool {
    pub pool: Address,
    pub config: Address,
    pub creator: Address,
    pub base_mint: Address,
    pub pool_type: u8,
    pub activation_point: u64,
}

impl EvtInitializePool {
    pub const DISCRIMINATOR: [u8; 8] = [228, 50, 246, 85, 203, 66, 134, 37];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
