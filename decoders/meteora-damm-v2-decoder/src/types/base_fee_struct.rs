use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::BaseFeeInfo;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BaseFeeStruct {
    pub base_fee_info: BaseFeeInfo,
    pub padding_1: u64,
}
