use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreatePosition {
    pub pool: Address,
    pub owner: Address,
    pub position: Address,
    pub position_nft_mint: Address,
}

impl EvtCreatePosition {
    pub const DISCRIMINATOR: [u8; 8] = [156, 15, 119, 198, 29, 181, 221, 55];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
