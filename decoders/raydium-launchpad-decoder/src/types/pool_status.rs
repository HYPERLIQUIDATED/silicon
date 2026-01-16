use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PoolStatus {
    Fund,
    Migrate,
    Trade,
}
