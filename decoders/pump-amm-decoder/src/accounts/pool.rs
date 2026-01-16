use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Pool {
    pub pool_bump: u8,
    pub index: u16,
    pub creator: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub lp_mint: Address,
    pub pool_base_token_account: Address,
    pub pool_quote_token_account: Address,
    pub lp_supply: u64,
    pub coin_creator: Address,
    pub is_mayhem_mode: bool,
}

impl Pool {
    pub const DISCRIMINATOR: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
