use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::SwapParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Swap {
    pub params: SwapParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwapInstructionAccounts {
    pub pool_authority: Address,
    pub config: Address,
    pub pool: Address,
    pub input_token_account: Address,
    pub output_token_account: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub payer: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub referral_token_account: Option<Address>,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Swap {
    pub const DISCRIMINATOR: [u8; 8] = [248, 198, 158, 145, 225, 117, 135, 200];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let referral_token_account = next_account(&mut iter);
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SwapInstructionAccounts {
            pool_authority,
            config,
            pool,
            input_token_account,
            output_token_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            payer,
            token_base_program,
            token_quote_program,
            referral_token_account,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
