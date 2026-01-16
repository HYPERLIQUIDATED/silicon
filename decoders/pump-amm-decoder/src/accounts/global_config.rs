use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct GlobalConfig {
    pub admin: Address,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Address; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Address,
    pub whitelist_pda: Address,
    pub reserved_fee_recipient: Address,
    pub mayhem_mode_enabled: bool,
    pub reserved_fee_recipients: [Address; 7],
}

impl GlobalConfig {
    pub const DISCRIMINATOR: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
