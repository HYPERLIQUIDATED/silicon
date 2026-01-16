use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeAuthority {
    pub claimer_a: Address,
    pub claimer_b: Address,
    pub claimer_a_bps: u16,
    pub claimer_b_bps: u16,
    pub mint: Address,
    pub bump: u8,
}

impl FeeAuthority {
    pub const DISCRIMINATOR: [u8; 8] = [135, 162, 91, 24, 156, 94, 193, 104];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
