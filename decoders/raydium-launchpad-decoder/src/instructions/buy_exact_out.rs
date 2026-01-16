use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BuyExactOut {
    pub amount_out: u64,
    pub maximum_amount_in: u64,
    pub share_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyExactOutInstructionAccounts {
    pub payer: Address,
    pub authority: Address,
    pub global_config: Address,
    pub platform_config: Address,
    pub pool_state: Address,
    pub user_base_token: Address,
    pub user_quote_token: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub base_token_mint: Address,
    pub quote_token_mint: Address,
    pub base_token_program: Address,
    pub quote_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl BuyExactOut {
    pub const DISCRIMINATOR: [u8; 8] = [24, 211, 116, 40, 105, 3, 153, 56];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for BuyExactOut {
    type ArrangedAccounts = BuyExactOutInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let user_base_token = next_account(&mut iter)?;
        let user_quote_token = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let base_token_mint = next_account(&mut iter)?;
        let quote_token_mint = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(BuyExactOutInstructionAccounts {
            payer,
            authority,
            global_config,
            platform_config,
            pool_state,
            user_base_token,
            user_quote_token,
            base_vault,
            quote_vault,
            base_token_mint,
            quote_token_mint,
            base_token_program,
            quote_token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
