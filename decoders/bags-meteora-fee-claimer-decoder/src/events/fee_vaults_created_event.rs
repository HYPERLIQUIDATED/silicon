use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeVaultsCreatedEvent {
    pub fee_authority: Address,
    pub base_mint: Address,
    pub claimer_a: Address,
    pub claimer_b: Address,
    pub claimer_a_bps: u16,
    pub claimer_b_bps: u16,
}

impl FeeVaultsCreatedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [29, 33, 12, 23, 95, 249, 238, 190];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
