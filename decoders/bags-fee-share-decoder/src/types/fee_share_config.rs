use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeShareConfig {
    pub base_mint: Address,
    pub quote_mint: Address,
    pub partner: Address,
    pub partner_config: Address,
    pub padding_0: [u64; 5],
    pub is_init_finalized: u8,
    pub is_update_locked: u8,
    pub padding_1: [u8; 5],
    pub bump: u8,
    pub claimers: [Address; 1],
    pub bps: [u16; 1],
}
