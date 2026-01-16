use borsh::{BorshDeserialize, BorshSerialize};

use crate::types::{BaseFeeStruct, DynamicFeeStruct};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolFeesStruct {
    pub base_fee: BaseFeeStruct,
    pub protocol_fee_percent: u8,
    pub partner_fee_percent: u8,
    pub referral_fee_percent: u8,
    pub padding_0: [u8; 5],
    pub dynamic_fee: DynamicFeeStruct,
    pub init_sqrt_price: u128,
}
