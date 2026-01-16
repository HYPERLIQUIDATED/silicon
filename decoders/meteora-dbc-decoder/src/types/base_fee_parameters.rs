use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BaseFeeParameters {
    pub cliff_fee_numerator: u64,
    pub first_factor: u16,
    pub second_factor: u64,
    pub third_factor: u64,
    pub base_fee_mode: u8,
}
