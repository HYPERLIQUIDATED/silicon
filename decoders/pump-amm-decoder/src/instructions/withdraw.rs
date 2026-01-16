use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Withdraw {
    pub lp_token_amount_in: u64,
    pub min_base_amount_out: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawInstructionAccounts {
    pub pool: Address,
    pub global_config: Address,
    pub user: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub lp_mint: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub user_pool_token_account: Address,
    pub pool_base_token_account: Address,
    pub pool_quote_token_account: Address,
    pub token_program: Address,
    pub token2022_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Withdraw {
    pub const DISCRIMINATOR: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Withdraw {
    type ArrangedAccounts = WithdrawInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let user_base_token_account = next_account(&mut iter)?;
        let user_quote_token_account = next_account(&mut iter)?;
        let user_pool_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(WithdrawInstructionAccounts {
            pool,
            global_config,
            user,
            base_mint,
            quote_mint,
            lp_mint,
            user_base_token_account,
            user_quote_token_account,
            user_pool_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            token_program,
            token2022_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
