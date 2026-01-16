use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePartnerFeeCollectionParameters {
    pub fee_collection_mode: u8,
    pub fee_collection_platform_bps: u16,
    pub fee_collection_claimers_bps: u16,
}
