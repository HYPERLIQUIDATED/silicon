use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetParams {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: Address,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee_basis_points: u64,
    pub set_creator_authority: Address,
    pub admin_set_creator_authority: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetParamsInstructionAccounts {
    pub global: Address,
    pub authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetParams {
    pub const DISCRIMINATOR: [u8; 8] = [27, 234, 178, 52, 147, 2, 187, 141];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetParams {
    type ArrangedAccounts = SetParamsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetParamsInstructionAccounts {
            global,
            authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
