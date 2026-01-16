use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SellEvent {
    pub timestamp: i64,
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub quote_amount_out: u64,
    pub lp_fee_basis_points: u64,
    pub lp_fee: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee: u64,
    pub quote_amount_out_without_lp_fee: u64,
    pub user_quote_amount_out: u64,
    pub pool: Address,
    pub user: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub protocol_fee_recipient: Address,
    pub protocol_fee_recipient_token_account: Address,
    pub coin_creator: Address,
    pub coin_creator_fee_basis_points: u64,
    pub coin_creator_fee: u64,
}

impl SellEvent {
    pub const DISCRIMINATOR: [u8; 8] = [62, 47, 55, 10, 165, 3, 220, 42];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
