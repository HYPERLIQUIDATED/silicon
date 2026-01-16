use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimCreatorTradingFee {
    pub max_base_amount: u64,
    pub max_quote_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimCreatorTradingFeeInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub token_a_account: Address,
    pub token_b_account: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub creator: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimCreatorTradingFee {
    pub const DISCRIMINATOR: [u8; 8] = [82, 220, 250, 189, 3, 85, 107, 45];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimCreatorTradingFee {
    type ArrangedAccounts = ClaimCreatorTradingFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let token_a_account = next_account(&mut iter)?;
        let token_b_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimCreatorTradingFeeInstructionAccounts {
            pool_authority,
            pool,
            token_a_account,
            token_b_account,
            base_vault,
            quote_vault,
            base_mint,
            quote_mint,
            creator,
            token_base_program,
            token_quote_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
