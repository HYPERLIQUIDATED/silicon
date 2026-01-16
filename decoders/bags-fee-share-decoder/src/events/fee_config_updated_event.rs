use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeConfigUpdatedEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub fee_config: Address,
    pub fee_authority: Address,
    pub base_mint: Address,
    pub new_bps: Vec<u16>,
    pub new_claimers: Vec<Address>,
}

impl FeeConfigUpdatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [139, 188, 235, 116, 222, 55, 95, 201];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
