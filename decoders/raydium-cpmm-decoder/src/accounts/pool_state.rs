use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolState {
    pub amm_config: Address,
    pub pool_creator: Address,
    pub token_0_vault: Address,
    pub token_1_vault: Address,
    pub lp_mint: Address,
    pub token_0_mint: Address,
    pub token_1_mint: Address,
    pub token_0_program: Address,
    pub token_1_program: Address,
    pub observation_key: Address,
    pub auth_bump: u8,
    pub status: u8,
    pub lp_mint_decimals: u8,
    pub mint_0_decimals: u8,
    pub mint_1_decimals: u8,
    pub lp_supply: u64,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub creator_fee_on: u8,
    pub enable_creator_fee: bool,
    pub padding1: [u8; 6],
    pub creator_fees_token_0: u64,
    pub creator_fees_token_1: u64,
    pub padding: [u64; 28],
}

impl PoolState {
    pub const DISCRIMINATOR: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
