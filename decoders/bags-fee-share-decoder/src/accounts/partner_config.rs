use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PartnerConfig {
    pub total_claimed_fees: u128,
    pub total_accumulated_fees: u64,
    pub total_lifetime_accumulated_fees: u128,
    pub partner: Address,
    pub fee_collection_mode: u8,
    pub fee_collection_platform_bps: u16,
    pub padding_1: [u8; 2],
    pub bump: u8,
    pub bps: u16,
    pub padding_0: [u64; 5],
}

impl PartnerConfig {
    pub const DISCRIMINATOR: [u8; 8] = [212, 110, 106, 253, 66, 131, 77, 96];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
