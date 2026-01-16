use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::ConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateConfigV2 {
    pub config: Address,
    pub quote_mint: Address,
    pub fee_claimer: Address,
    pub leftover_receiver: Address,
    pub config_parameters: ConfigParameters,
}

impl EvtCreateConfigV2 {
    pub const DISCRIMINATOR: [u8; 8] = [163, 74, 66, 187, 119, 195, 26, 144];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
