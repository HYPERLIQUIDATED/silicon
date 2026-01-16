use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LpChangeEvent {
    pub pool_id: Address,
    pub lp_amount_before: u64,
    pub token_0_vault_before: u64,
    pub token_1_vault_before: u64,
    pub token_0_amount: u64,
    pub token_1_amount: u64,
    pub token_0_transfer_fee: u64,
    pub token_1_transfer_fee: u64,
    pub change_type: u8,
}

impl LpChangeEvent {
    pub const DISCRIMINATOR: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
