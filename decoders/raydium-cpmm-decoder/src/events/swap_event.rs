use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SwapEvent {
    pub pool_id: Address,
    pub input_vault_before: u64,
    pub output_vault_before: u64,
    pub input_amount: u64,
    pub output_amount: u64,
    pub input_transfer_fee: u64,
    pub output_transfer_fee: u64,
    pub base_input: bool,
    pub input_mint: Address,
    pub output_mint: Address,
    pub trade_fee: u64,
    pub creator_fee: u64,
    pub creator_fee_on_input: bool,
}

impl SwapEvent {
    pub const DISCRIMINATOR: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
