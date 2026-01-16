use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::ClaimProtocol;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BagsFeeShareProtocolClaimEvent {
    pub timestamp: i64,
    pub payer: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub pool: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub claimers: Vec<Address>,
    pub fees: Vec<u64>,
    pub protocol: ClaimProtocol,
}

impl BagsFeeShareProtocolClaimEvent {
    pub const DISCRIMINATOR: [u8; 8] = [168, 122, 235, 187, 157, 4, 94, 126];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
