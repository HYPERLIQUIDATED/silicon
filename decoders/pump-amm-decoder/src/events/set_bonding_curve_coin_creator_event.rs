use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetBondingCurveCoinCreatorEvent {
    pub timestamp: i64,
    pub base_mint: Address,
    pub pool: Address,
    pub bonding_curve: Address,
    pub coin_creator: Address,
}

impl SetBondingCurveCoinCreatorEvent {
    pub const DISCRIMINATOR: [u8; 8] = [242, 231, 235, 102, 65, 99, 189, 211];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
