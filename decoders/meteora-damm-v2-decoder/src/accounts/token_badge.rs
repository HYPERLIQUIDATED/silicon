use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TokenBadge {
    pub token_mint: Address,
    pub _padding: [u8; 128],
}

impl TokenBadge {
    pub const DISCRIMINATOR: [u8; 8] = [116, 219, 204, 229, 249, 116, 255, 150];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
