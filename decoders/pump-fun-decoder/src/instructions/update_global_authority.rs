use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateGlobalAuthority {}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateGlobalAuthorityInstructionAccounts {
    pub global: Address,
    pub authority: Address,
    pub new_authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateGlobalAuthority {
    pub const DISCRIMINATOR: [u8; 8] = [227, 181, 74, 196, 208, 21, 97, 213];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateGlobalAuthority {
    type ArrangedAccounts = UpdateGlobalAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let new_authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateGlobalAuthorityInstructionAccounts {
            global,
            authority,
            new_authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
