use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ExtendCreatedFeeConfigParameters {
    pub bps: Vec<u16>,
    pub finalize_init: bool,
}
