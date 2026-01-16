use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetMetaplexCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Address,
    pub pool: Address,
    pub metadata: Address,
    pub coin_creator: Address,
}

impl SetMetaplexCoinCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [150, 107, 199, 123, 124, 207, 102, 228];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
