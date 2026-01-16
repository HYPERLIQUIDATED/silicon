use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeShareConfigHeader {
    pub base_mint: Address,
    pub quote_mint: Address,
    pub partner: Address,
    pub partner_config: Address,
    pub padding_0: [u64; 5],
    pub is_init_finalized: u8,
    pub is_update_locked: u8,
    pub padding_1: [u8; 5],
    pub bump: u8,
}

impl FeeShareConfigHeader {
    pub const DISCRIMINATOR: [u8; 8] = [40, 71, 136, 156, 222, 49, 31, 201];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
