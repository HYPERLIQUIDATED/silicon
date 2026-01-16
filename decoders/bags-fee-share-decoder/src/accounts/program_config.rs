use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ProgramConfig {
    pub admin: Address,
    pub pending_admin: Option<Address>,
    pub platform_vault: Address,
    pub platform_bps: u16,
    pub total_claimed_fees: u128,
    pub total_accumulated_fees: u64,
    pub total_lifetime_accumulated_fees: u128,
    pub padding_0: [u64; 5],
    pub padding_1: [u8; 7],
    pub bump: u8,
}

impl ProgramConfig {
    pub const DISCRIMINATOR: [u8; 8] = [196, 210, 90, 231, 144, 149, 140, 63];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
