use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{ConfigStatus, Shareholder};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SharingConfig {
    pub bump: u8,
    pub version: u8,
    pub status: ConfigStatus,
    pub mint: Address,
    pub admin: Address,
    pub admin_revoked: bool,
    pub shareholders: Vec<Shareholder>,
}

impl SharingConfig {
    pub const DISCRIMINATOR: [u8; 8] = [216, 74, 9, 0, 56, 140, 93, 75];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
