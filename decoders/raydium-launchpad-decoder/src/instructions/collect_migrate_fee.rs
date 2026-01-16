use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectMigrateFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectMigrateFeeInstructionAccounts {
    pub owner: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub global_config: Address,
    pub quote_vault: Address,
    pub quote_mint: Address,
    pub recipient_token_account: Address,
    pub token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CollectMigrateFee {
    pub const DISCRIMINATOR: [u8; 8] = [255, 186, 150, 223, 235, 118, 201, 186];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CollectMigrateFee {
    type ArrangedAccounts = CollectMigrateFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CollectMigrateFeeInstructionAccounts {
            owner,
            authority,
            pool_state,
            global_config,
            quote_vault,
            quote_mint,
            recipient_token_account,
            token_program,
            remaining: remaining.to_vec(),
        })
    }
}
