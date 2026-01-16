use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializePoolParameters {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
