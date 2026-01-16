use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClosePosition {
    pub pool: Address,
    pub owner: Address,
    pub position: Address,
    pub position_nft_mint: Address,
}

impl EvtClosePosition {
    pub const DISCRIMINATOR: [u8; 8] = [20, 145, 144, 68, 143, 142, 214, 178];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
