use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TokenSupplyParams {
    pub pre_migration_token_supply: u64,
    pub post_migration_token_supply: u64,
}
