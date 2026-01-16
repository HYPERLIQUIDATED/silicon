use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateFeeConfigParameters {
    pub bps: Vec<u16>,
    pub finalize_update: bool,
    pub from_idx: u8,
    pub to_idx: u8,
}
