use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::InitializePoolParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeVirtualPoolWithToken2022 {
    pub params: InitializePoolParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeVirtualPoolWithToken2022InstructionAccounts {
    pub config: Address,
    pub pool_authority: Address,
    pub creator: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub pool: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub payer: Address,
    pub token_quote_program: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeVirtualPoolWithToken2022 {
    pub const DISCRIMINATOR: [u8; 8] = [169, 118, 51, 78, 145, 110, 220, 155];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeVirtualPoolWithToken2022 {
    type ArrangedAccounts = InitializeVirtualPoolWithToken2022InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeVirtualPoolWithToken2022InstructionAccounts {
            config,
            pool_authority,
            creator,
            base_mint,
            quote_mint,
            pool,
            base_vault,
            quote_vault,
            payer,
            token_quote_program,
            token_program,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
