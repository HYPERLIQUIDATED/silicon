use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TransferFeeExtensionParams {
    pub transfer_fee_basis_points: u16,
    pub maximum_fee: u64,
}
