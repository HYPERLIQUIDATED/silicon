use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{FeeTier, Fees};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeConfig {
    pub bump: u8,
    pub admin: Address,
    pub flat_fees: Fees,
    pub fee_tiers: Vec<FeeTier>,
}

impl FeeConfig {
    pub const DISCRIMINATOR: [u8; 8] = [143, 52, 146, 187, 219, 123, 76, 155];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
