use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePoolEvent {
    pub timestamp: i64,
    pub index: u16,
    pub creator: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub base_mint_decimals: u8,
    pub quote_mint_decimals: u8,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub pool_base_amount: u64,
    pub pool_quote_amount: u64,
    pub minimum_liquidity: u64,
    pub initial_liquidity: u64,
    pub lp_token_amount_out: u64,
    pub pool_bump: u8,
    pub pool: Address,
    pub lp_mint: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub coin_creator: Address,
    pub is_mayhem_mode: bool,
}

impl CreatePoolEvent {
    pub const DISCRIMINATOR: [u8; 8] = [177, 49, 12, 210, 160, 118, 167, 116];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
