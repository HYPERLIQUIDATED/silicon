use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimTokenIncentivesEvent {
    pub user: Address,
    pub mint: Address,
    pub amount: u64,
    pub timestamp: i64,
    pub total_claimed_tokens: u64,
    pub current_sol_volume: u64,
}

impl ClaimTokenIncentivesEvent {
    pub const DISCRIMINATOR: [u8; 8] = [79, 172, 246, 49, 205, 91, 206, 232];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
