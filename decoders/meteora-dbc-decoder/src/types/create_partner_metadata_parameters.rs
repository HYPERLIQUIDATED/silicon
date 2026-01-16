use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePartnerMetadataParameters {
    pub padding: [u8; 96],
    pub name: String,
    pub website: String,
    pub logo: String,
}
