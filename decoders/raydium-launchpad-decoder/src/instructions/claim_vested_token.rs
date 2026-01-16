use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimVestedToken {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimVestedTokenInstructionAccounts {
    pub beneficiary: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub vesting_record: Address,
    pub base_vault: Address,
    pub user_base_token: Address,
    pub base_token_mint: Address,
    pub base_token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimVestedToken {
    pub const DISCRIMINATOR: [u8; 8] = [49, 33, 104, 30, 189, 157, 79, 35];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimVestedToken {
    type ArrangedAccounts = ClaimVestedTokenInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let beneficiary = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let vesting_record = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let user_base_token = next_account(&mut iter)?;
        let base_token_mint = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimVestedTokenInstructionAccounts {
            beneficiary,
            authority,
            pool_state,
            vesting_record,
            base_vault,
            user_base_token,
            base_token_mint,
            base_token_program,
            system_program,
            associated_token_program,
            remaining: remaining.to_vec(),
        })
    }
}
