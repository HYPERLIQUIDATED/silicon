use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Vault {
    pub authority: Address,
    pub claimer: Address,
    pub bump: u8,
}

impl Vault {
    pub const DISCRIMINATOR: [u8; 8] = [211, 8, 232, 43, 2, 152, 117, 119];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
