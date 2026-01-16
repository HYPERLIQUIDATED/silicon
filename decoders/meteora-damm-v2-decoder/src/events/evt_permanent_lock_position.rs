use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtPermanentLockPosition {
    pub pool: Address,
    pub position: Address,
    pub lock_liquidity_amount: u128,
    pub total_permanent_locked_liquidity: u128,
}

impl EvtPermanentLockPosition {
    pub const DISCRIMINATOR: [u8; 8] = [145, 143, 162, 218, 218, 80, 67, 11];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
