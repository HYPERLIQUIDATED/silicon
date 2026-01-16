use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateBondingCurveCreatorEvent {
    pub timestamp: i64,
    pub mint: Address,
    pub bonding_curve: Address,
    pub sharing_config: Address,
    pub old_creator: Address,
    pub new_creator: Address,
}

impl MigrateBondingCurveCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [155, 167, 104, 220, 213, 108, 243, 3];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
