use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateMayhemVirtualParamsEvent {
    pub timestamp: i64,
    pub mint: Address,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub new_virtual_token_reserves: u64,
    pub new_virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
}

impl UpdateMayhemVirtualParamsEvent {
    pub const DISCRIMINATOR: [u8; 8] = [117, 123, 228, 182, 161, 168, 220, 214];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
