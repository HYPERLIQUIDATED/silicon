use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePool {
    pub index: u16,
    pub base_amount_in: u64,
    pub quote_amount_in: u64,
    pub coin_creator: Address,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePoolInstructionAccounts {
    pub pool: Address,
    pub global_config: Address,
    pub creator: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub lp_mint: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub user_pool_token_account: Address,
    pub pool_base_token_account: Address,
    pub pool_quote_token_account: Address,
    pub system_program: Address,
    pub token2022_program: Address,
    pub base_token_program: Address,
    pub quote_token_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePool {
    pub const DISCRIMINATOR: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePool {
    type ArrangedAccounts = CreatePoolInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let user_base_token_account = next_account(&mut iter)?;
        let user_quote_token_account = next_account(&mut iter)?;
        let user_pool_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePoolInstructionAccounts {
            pool,
            global_config,
            creator,
            base_mint,
            quote_mint,
            lp_mint,
            user_base_token_account,
            user_quote_token_account,
            user_pool_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            system_program,
            token2022_program,
            base_token_program,
            quote_token_program,
            associated_token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
