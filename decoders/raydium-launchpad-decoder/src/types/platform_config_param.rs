use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{MigrateNftInfo, PlatformConfigInfo};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PlatformConfigParam {
    FeeWallet(Address),
    NFTWallet(Address),
    MigrateNftInfo(MigrateNftInfo),
    FeeRate(u64),
    Name(String),
    Web(String),
    Img(String),
    CpSwapConfig,
    AllInfo(PlatformConfigInfo),
    VestingWallet(Address),
    PlatformVestingScale(u64),
    PlatformCPCreator(Address),
}
