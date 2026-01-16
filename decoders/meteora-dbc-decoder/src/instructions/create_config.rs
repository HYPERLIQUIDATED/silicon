use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::ConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateConfig {
    pub config_parameters: ConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateConfigInstructionAccounts {
    pub config: Address,
    pub fee_claimer: Address,
    pub leftover_receiver: Address,
    pub quote_mint: Address,
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
        let fee_claimer = next_account(&mut iter)?;
        let leftover_receiver = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateConfigInstructionAccounts {
            config,
            fee_claimer,
            leftover_receiver,
            quote_mint,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
