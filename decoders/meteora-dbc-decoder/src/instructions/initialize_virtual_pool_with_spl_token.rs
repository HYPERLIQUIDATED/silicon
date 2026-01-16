use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::InitializePoolParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeVirtualPoolWithSplToken {
    pub params: InitializePoolParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeVirtualPoolWithSplTokenInstructionAccounts {
    pub config: Address,
    pub pool_authority: Address,
    pub creator: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub pool: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub mint_metadata: Address,
    pub metadata_program: Address,
    pub payer: Address,
    pub token_quote_program: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeVirtualPoolWithSplToken {
    pub const DISCRIMINATOR: [u8; 8] = [140, 85, 215, 176, 102, 54, 104, 79];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeVirtualPoolWithSplToken {
    type ArrangedAccounts = InitializeVirtualPoolWithSplTokenInstructionAccounts;

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
        let mint_metadata = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeVirtualPoolWithSplTokenInstructionAccounts {
            config,
            pool_authority,
            creator,
            base_mint,
            quote_mint,
            pool,
            base_vault,
            quote_vault,
            mint_metadata,
            metadata_program,
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
