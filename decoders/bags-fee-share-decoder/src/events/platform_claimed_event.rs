use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformClaimedEvent {
    pub timestamp: i64,
    pub platform_vault: Address,
    pub admin: Address,
    pub claimed: u64,
    pub total_claimed_fees: u128,
}

impl PlatformClaimedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [147, 166, 33, 185, 161, 49, 207, 139];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
