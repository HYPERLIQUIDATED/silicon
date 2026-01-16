use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCreateTokenBadge {
    pub token_mint: Address,
}

impl EvtCreateTokenBadge {
    pub const DISCRIMINATOR: [u8; 8] = [141, 120, 134, 116, 34, 28, 114, 160];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
