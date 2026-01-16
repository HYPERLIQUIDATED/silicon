use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{BaseFeeParameters, DynamicFeeParameters};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolFeeParameters {
    pub base_fee: BaseFeeParameters,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
