use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::CreateFeeVaultsParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateFeeVaults {
    pub input: CreateFeeVaultsParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateFeeVaultsInstructionAccounts {
    pub payer: Address,
    pub authority: Address,
    pub claimer_a: Address,
    pub claimer_b: Address,
    pub vault_a: Address,
    pub vault_b: Address,
    pub fee_authority: Address,
    pub fee_authority_quote_ata: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateFeeVaults {
    pub const DISCRIMINATOR: [u8; 8] = [40, 216, 239, 141, 127, 220, 173, 221];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateFeeVaults {
    type ArrangedAccounts = CreateFeeVaultsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let claimer_a = next_account(&mut iter)?;
        let claimer_b = next_account(&mut iter)?;
        let vault_a = next_account(&mut iter)?;
        let vault_b = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let fee_authority_quote_ata = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateFeeVaultsInstructionAccounts {
            payer,
            authority,
            claimer_a,
            claimer_b,
            vault_a,
            vault_b,
            fee_authority,
            fee_authority_quote_ata,
            base_mint,
            quote_mint,
            system_program,
            token_program,
            associated_token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
