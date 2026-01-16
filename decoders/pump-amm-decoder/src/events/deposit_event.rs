use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DepositEvent {
    pub timestamp: i64,
    pub lp_token_amount_out: u64,
    pub max_base_amount_in: u64,
    pub max_quote_amount_in: u64,
    pub user_base_token_reserves: u64,
    pub user_quote_token_reserves: u64,
    pub pool_base_token_reserves: u64,
    pub pool_quote_token_reserves: u64,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub lp_mint_supply: u64,
    pub pool: Address,
    pub user: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub user_pool_token_account: Address,
}

impl DepositEvent {
    pub const DISCRIMINATOR: [u8; 8] = [120, 248, 61, 83, 31, 142, 107, 144];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
