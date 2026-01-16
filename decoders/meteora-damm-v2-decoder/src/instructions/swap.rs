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
    pub pool: Address,
    pub input_token_account: Address,
    pub output_token_account: Address,
    pub token_a_vault: Address,
    pub token_b_vault: Address,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub payer: Address,
    pub token_a_program: Address,
    pub token_b_program: Address,
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
        let pool = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_a_program = next_account(&mut iter)?;
        let token_b_program = next_account(&mut iter)?;
        let referral_token_account = next_account(&mut iter);
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SwapInstructionAccounts {
            pool_authority,
            pool,
            input_token_account,
            output_token_account,
            token_a_vault,
            token_b_vault,
            token_a_mint,
            token_b_mint,
            payer,
            token_a_program,
            token_b_program,
            referral_token_account,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
