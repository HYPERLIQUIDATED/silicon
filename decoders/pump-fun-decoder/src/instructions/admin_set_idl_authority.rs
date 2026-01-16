use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetIdlAuthority {
    pub idl_authority: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdminSetIdlAuthorityInstructionAccounts {
    pub authority: Address,
    pub global: Address,
    pub idl_account: Address,
    pub system_program: Address,
    pub program_signer: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl AdminSetIdlAuthority {
    pub const DISCRIMINATOR: [u8; 8] = [8, 217, 96, 231, 144, 104, 192, 5];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for AdminSetIdlAuthority {
    type ArrangedAccounts = AdminSetIdlAuthorityInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let idl_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let program_signer = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(AdminSetIdlAuthorityInstructionAccounts {
            authority,
            global,
            idl_account,
            system_program,
            program_signer,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
