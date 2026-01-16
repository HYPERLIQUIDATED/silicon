use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrationFee {
    pub fee_percentage: u8,
    pub creator_fee_percentage: u8,
}
