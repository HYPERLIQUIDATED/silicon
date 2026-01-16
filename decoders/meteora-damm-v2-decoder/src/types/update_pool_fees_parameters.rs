use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::DynamicFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePoolFeesParameters {
    pub cliff_fee_numerator: Option<u64>,
    pub dynamic_fee: Option<DynamicFeeParameters>,
}
