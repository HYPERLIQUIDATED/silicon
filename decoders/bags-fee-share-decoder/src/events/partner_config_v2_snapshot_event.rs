use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PartnerConfigV2SnapshotEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub partner_config: Address,
    pub partner: Address,
    pub fee_collection_mode: u8,
    pub fee_collection_platform_bps: u16,
    pub fee_collection_claimers_bps: u16,
    pub padding_0: [u64; 5],
}

impl PartnerConfigV2SnapshotEvent {
    pub const DISCRIMINATOR: [u8; 8] = [44, 79, 22, 247, 183, 216, 168, 183];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
