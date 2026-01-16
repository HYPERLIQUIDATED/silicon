use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateFeeVaultsParameters {
    pub claimer_a_bps: u16,
    pub claimer_b_bps: u16,
}
