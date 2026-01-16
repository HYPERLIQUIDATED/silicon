use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Initialize {}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeInstructionAccounts {
    pub global: Address,
    pub user: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Initialize {
    pub const DISCRIMINATOR: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Initialize {
    type ArrangedAccounts = InitializeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeInstructionAccounts {
            global,
            user,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
