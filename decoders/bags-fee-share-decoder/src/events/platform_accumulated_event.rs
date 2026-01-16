use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::ClaimProtocol;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformAccumulatedEvent {
    pub timestamp: i64,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub pool: Address,
    pub platform_vault: Address,
    pub accumulated: u64,
    pub total_accumulated_fees: u64,
    pub total_lifetime_accumulated_fees: u128,
    pub protocol: ClaimProtocol,
}

impl PlatformAccumulatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [135, 109, 121, 126, 171, 62, 118, 76];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
