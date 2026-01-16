use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimTradingFee {
    pub max_amount_a: u64,
    pub max_amount_b: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimTradingFeeInstructionAccounts {
    pub pool_authority: Address,
    pub config: Address,
    pub pool: Address,
    pub token_a_account: Address,
    pub token_b_account: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub fee_claimer: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimTradingFee {
    pub const DISCRIMINATOR: [u8; 8] = [8, 236, 89, 49, 152, 125, 177, 81];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimTradingFee {
    type ArrangedAccounts = ClaimTradingFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let token_a_account = next_account(&mut iter)?;
        let token_b_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let fee_claimer = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimTradingFeeInstructionAccounts {
            pool_authority,
            config,
            pool,
            token_a_account,
            token_b_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            fee_claimer,
            token_base_program,
            token_quote_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
