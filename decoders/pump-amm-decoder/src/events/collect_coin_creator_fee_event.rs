use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectCoinCreatorFeeEvent {
    pub timestamp: i64,
    pub coin_creator: Address,
    pub coin_creator_fee: u64,
    pub coin_creator_vault_ata: Address,
    pub coin_creator_token_account: Address,
}

impl CollectCoinCreatorFeeEvent {
    pub const DISCRIMINATOR: [u8; 8] = [232, 245, 194, 238, 234, 218, 58, 89];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
