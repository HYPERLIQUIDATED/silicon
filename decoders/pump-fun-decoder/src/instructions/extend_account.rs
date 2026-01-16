use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ExtendAccount {}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtendAccountInstructionAccounts {
    pub account: Address,
    pub user: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ExtendAccount {
    pub const DISCRIMINATOR: [u8; 8] = [234, 102, 194, 203, 150, 72, 62, 229];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ExtendAccount {
    type ArrangedAccounts = ExtendAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let account = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ExtendAccountInstructionAccounts {
            account,
            user,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
