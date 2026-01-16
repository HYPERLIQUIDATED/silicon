use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateProgramConfigParameters {
    pub pending_admin: Option<Address>,
    pub platform_bps: Option<u16>,
}
