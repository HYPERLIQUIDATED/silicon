use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MeteoraDammMigrationMetadata {
    pub virtual_pool: Address,
    pub padding_0: [u8; 32],
    pub partner: Address,
    pub lp_mint: Address,
    pub partner_locked_liquidity: u64,
    pub partner_liquidity: u64,
    pub creator_locked_liquidity: u64,
    pub creator_liquidity: u64,
    pub _padding_0: u8,
    pub creator_locked_status: u8,
    pub partner_locked_status: u8,
    pub creator_claim_status: u8,
    pub partner_claim_status: u8,
    pub _padding: [u8; 107],
}

impl MeteoraDammMigrationMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [17, 155, 141, 215, 207, 4, 133, 156];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
