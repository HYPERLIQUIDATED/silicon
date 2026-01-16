use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigratePoolCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Address,
    pub pool: Address,
    pub sharing_config: Address,
    pub old_coin_creator: Address,
    pub new_coin_creator: Address,
}

impl MigratePoolCoinCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [170, 221, 82, 199, 147, 165, 247, 46];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
