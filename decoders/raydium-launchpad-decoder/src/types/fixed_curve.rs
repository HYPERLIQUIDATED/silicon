use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FixedCurve {
    pub supply: u64,
    pub total_quote_fund_raising: u64,
    pub migrate_type: u8,
}
