use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetCoinCreatorEvent {
    pub timestamp: i64,
    pub admin_set_coin_creator_authority: Address,
    pub base_mint: Address,
    pub pool: Address,
    pub old_coin_creator: Address,
    pub new_coin_creator: Address,
}

impl AdminSetCoinCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [45, 220, 93, 24, 25, 97, 172, 104];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
