use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::MigrateNftInfo;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformParams {
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
    pub creator_fee_rate: u64,
    pub platform_vesting_scale: u64,
}
