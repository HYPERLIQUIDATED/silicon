use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::StaticConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateConfig {
    pub index: u64,
    pub config_parameters: StaticConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateConfigInstructionAccounts {
    pub config: Address,
    pub operator: Address,
    pub signer: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateConfig {
    pub const DISCRIMINATOR: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let config = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateConfigInstructionAccounts {
            config,
            operator,
            signer,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
