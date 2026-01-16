use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateNftInfo {
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
}
