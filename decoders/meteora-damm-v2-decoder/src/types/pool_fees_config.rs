use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{BaseFeeInfo, DynamicFeeConfig};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeInfo,
    pub dynamic_fee: DynamicFeeConfig,
    pub protocol_fee_percent: u8,
    pub partner_fee_percent: u8,
    pub referral_fee_percent: u8,
    pub padding_0: [u8; 5],
    pub padding_1: [u64; 5],
}
