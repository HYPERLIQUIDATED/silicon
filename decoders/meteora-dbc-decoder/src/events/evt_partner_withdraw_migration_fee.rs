use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtPartnerWithdrawMigrationFee {
    pub pool: Address,
    pub fee: u64,
}

impl EvtPartnerWithdrawMigrationFee {
    pub const DISCRIMINATOR: [u8; 8] = [181, 105, 127, 67, 8, 187, 120, 57];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
