use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateAdmin {}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAdminInstructionAccounts {
    pub admin: Address,
    pub global_config: Address,
    pub new_admin: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateAdmin {
    pub const DISCRIMINATOR: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateAdmin {
    type ArrangedAccounts = UpdateAdminInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let new_admin = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateAdminInstructionAccounts {
            admin,
            global_config,
            new_admin,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
