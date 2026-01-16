use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{BaseFeeConfig, DynamicFeeConfig};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolFeesConfig {
    pub base_fee: BaseFeeConfig,
    pub dynamic_fee: DynamicFeeConfig,
}
