use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct VestingRecord {
    pub epoch: u64,
    pub pool: Address,
    pub beneficiary: Address,
    pub claimed_amount: u64,
    pub token_share_amount: u64,
    pub padding: [u64; 8],
}

impl VestingRecord {
    pub const DISCRIMINATOR: [u8; 8] = [106, 243, 221, 205, 230, 126, 85, 83];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
