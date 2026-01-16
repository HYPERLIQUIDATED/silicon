use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::UpdateProgramConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateProgramConfig {
    pub params: UpdateProgramConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateProgramConfigInstructionAccounts {
    pub admin: Address,
    pub program_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateProgramConfig {
    pub const DISCRIMINATOR: [u8; 8] = [214, 3, 187, 98, 170, 106, 33, 45];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateProgramConfig {
    type ArrangedAccounts = UpdateProgramConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateProgramConfigInstructionAccounts {
            admin,
            program_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
