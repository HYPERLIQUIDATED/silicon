use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatorWithdrawSurplus {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatorWithdrawSurplusInstructionAccounts {
    pub pool_authority: Address,
    pub config: Address,
    pub virtual_pool: Address,
    pub token_quote_account: Address,
    pub quote_vault: Address,
    pub quote_mint: Address,
    pub creator: Address,
    pub token_quote_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatorWithdrawSurplus {
    pub const DISCRIMINATOR: [u8; 8] = [165, 3, 137, 7, 28, 134, 76, 80];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatorWithdrawSurplus {
    type ArrangedAccounts = CreatorWithdrawSurplusInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let virtual_pool = next_account(&mut iter)?;
        let token_quote_account = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatorWithdrawSurplusInstructionAccounts {
            pool_authority,
            config,
            virtual_pool,
            token_quote_account,
            quote_vault,
            quote_mint,
            creator,
            token_quote_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
