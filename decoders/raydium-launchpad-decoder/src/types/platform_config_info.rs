use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::MigrateNftInfo;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformConfigInfo {
    pub fee_wallet: Address,
    pub nft_wallet: Address,
    pub migrate_nft_info: MigrateNftInfo,
    pub fee_rate: u64,
    pub name: String,
    pub web: String,
    pub img: String,
    pub transfer_fee_extension_auth: Address,
    pub creator_fee_rate: u64,
    pub platform_vesting_scale: u64,
    pub vesting_wallet: Address,
}
