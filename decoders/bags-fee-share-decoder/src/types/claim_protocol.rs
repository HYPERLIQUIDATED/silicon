use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum ClaimProtocol {
    Dbc,
    DammV2,
}
