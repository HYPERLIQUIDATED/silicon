use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{AmmCreatorFeeOn, VestingSchedule};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolState {
    pub epoch: u64,
    pub auth_bump: u8,
    pub status: u8,
    pub base_decimals: u8,
    pub quote_decimals: u8,
    pub migrate_type: u8,
    pub supply: u64,
    pub total_base_sell: u64,
    pub virtual_base: u64,
    pub virtual_quote: u64,
    pub real_base: u64,
    pub real_quote: u64,
    pub total_quote_fund_raising: u64,
    pub quote_protocol_fee: u64,
    pub platform_fee: u64,
    pub migrate_fee: u64,
    pub vesting_schedule: VestingSchedule,
    pub global_config: Address,
    pub platform_config: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub creator: Address,
    pub token_program_flag: u8,
    pub amm_creator_fee_on: AmmCreatorFeeOn,
    pub platform_vesting_share: u64,
    pub padding: [u8; 54],
}

impl PoolState {
    pub const DISCRIMINATOR: [u8; 8] = [247, 237, 227, 245, 215, 195, 222, 70];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
