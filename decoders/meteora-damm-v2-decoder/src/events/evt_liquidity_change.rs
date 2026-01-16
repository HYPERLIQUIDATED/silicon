use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtLiquidityChange {
    pub pool: Address,
    pub position: Address,
    pub owner: Address,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
    pub transfer_fee_included_token_a_amount: u64,
    pub transfer_fee_included_token_b_amount: u64,
    pub reserve_a_amount: u64,
    pub reserve_b_amount: u64,
    pub liquidity_delta: u128,
    pub token_a_amount_threshold: u64,
    pub token_b_amount_threshold: u64,
    pub change_type: u8,
}

impl EvtLiquidityChange {
    pub const DISCRIMINATOR: [u8; 8] = [197, 171, 78, 127, 224, 211, 87, 13];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
