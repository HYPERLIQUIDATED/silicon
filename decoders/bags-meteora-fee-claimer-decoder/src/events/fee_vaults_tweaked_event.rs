use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeVaultsTweakedEvent {
    pub fee_authority: Address,
    pub base_mint: Address,
    pub claimer_a: Address,
    pub claimer_b: Address,
    pub old_claimer_a_bps: u16,
    pub old_claimer_b_bps: u16,
    pub claimer_a_bps: u16,
    pub claimer_b_bps: u16,
}

impl FeeVaultsTweakedEvent {
    pub const DISCRIMINATOR: [u8; 8] = [118, 193, 160, 81, 236, 170, 187, 139];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
