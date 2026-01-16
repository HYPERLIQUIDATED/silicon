use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ConfirmAdmin {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmAdminInstructionAccounts {
    pub pending_admin: Address,
    pub program_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ConfirmAdmin {
    pub const DISCRIMINATOR: [u8; 8] = [18, 211, 32, 168, 193, 120, 133, 115];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ConfirmAdmin {
    type ArrangedAccounts = ConfirmAdminInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pending_admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ConfirmAdminInstructionAccounts {
            pending_admin,
            program_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
