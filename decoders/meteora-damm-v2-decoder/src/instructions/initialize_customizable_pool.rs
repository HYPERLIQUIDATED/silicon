use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::InitializeCustomizablePoolParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeCustomizablePool {
    pub params: InitializeCustomizablePoolParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeCustomizablePoolInstructionAccounts {
    pub creator: Address,
    pub position_nft_mint: Address,
    pub position_nft_account: Address,
    pub payer: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub position: Address,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub token_a_vault: Address,
    pub token_b_vault: Address,
    pub payer_token_a: Address,
    pub payer_token_b: Address,
    pub token_a_program: Address,
    pub token_b_program: Address,
    pub token2022_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeCustomizablePool {
    pub const DISCRIMINATOR: [u8; 8] = [20, 161, 241, 24, 189, 221, 180, 2];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeCustomizablePool {
    type ArrangedAccounts = InitializeCustomizablePoolInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let creator = next_account(&mut iter)?;
        let position_nft_mint = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let payer_token_a = next_account(&mut iter)?;
        let payer_token_b = next_account(&mut iter)?;
        let token_a_program = next_account(&mut iter)?;
        let token_b_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeCustomizablePoolInstructionAccounts {
            creator,
            position_nft_mint,
            position_nft_account,
            payer,
            pool_authority,
            pool,
            position,
            token_a_mint,
            token_b_mint,
            token_a_vault,
            token_b_vault,
            payer_token_a,
            payer_token_b,
            token_a_program,
            token_b_program,
            token2022_program,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
