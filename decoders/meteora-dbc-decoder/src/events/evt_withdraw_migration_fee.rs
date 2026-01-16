use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtWithdrawMigrationFee {
    pub pool: Address,
    pub fee: u64,
    pub flag: u8,
}

impl EvtWithdrawMigrationFee {
    pub const DISCRIMINATOR: [u8; 8] = [26, 203, 84, 85, 161, 23, 100, 214];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
