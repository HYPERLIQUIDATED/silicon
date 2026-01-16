use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Operator {
    pub whitelisted_address: Address,
    pub permission: u128,
    pub padding: [u64; 2],
}

impl Operator {
    pub const DISCRIMINATOR: [u8; 8] = [219, 31, 188, 145, 69, 139, 204, 117];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
