use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Global {
    pub initialized: bool,
    pub authority: Address,
    pub fee_recipient: Address,
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Address,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub fee_recipients: [Address; 7],
    pub set_creator_authority: Address,
    pub admin_set_creator_authority: Address,
    pub create_v2_enabled: bool,
    pub whitelist_pda: Address,
    pub reserved_fee_recipient: Address,
    pub mayhem_mode_enabled: bool,
    pub reserved_fee_recipients: [Address; 7],
}

impl Global {
    pub const DISCRIMINATOR: [u8; 8] = [167, 232, 232, 177, 200, 108, 114, 127];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
