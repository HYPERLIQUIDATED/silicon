use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::PlatformCurveParam;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformConfig {
    pub epoch: u64,
    pub platform_fee_wallet: Address,
    pub platform_nft_wallet: Address,
    pub platform_scale: u64,
    pub creator_scale: u64,
    pub burn_scale: u64,
    pub fee_rate: u64,
    pub name: [u8; 64],
    pub web: [u8; 256],
    pub img: [u8; 256],
    pub cpswap_config: Address,
    pub creator_fee_rate: u64,
    pub transfer_fee_extension_auth: Address,
    pub platform_vesting_wallet: Address,
    pub platform_vesting_scale: u64,
    pub platform_cp_creator: Address,
    pub padding: [u8; 108],
    pub curve_params: Vec<PlatformCurveParam>,
}

impl PlatformConfig {
    pub const DISCRIMINATOR: [u8; 8] = [160, 78, 128, 0, 248, 83, 230, 160];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
