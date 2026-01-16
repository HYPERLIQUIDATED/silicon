use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectCreatorFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectCreatorFeeInstructionAccounts {
    pub creator: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub amm_config: Address,
    pub token_0_vault: Address,
    pub token_1_vault: Address,
    pub vault_0_mint: Address,
    pub vault_1_mint: Address,
    pub creator_token_0: Address,
    pub creator_token_1: Address,
    pub token_0_program: Address,
    pub token_1_program: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CollectCreatorFee {
    pub const DISCRIMINATOR: [u8; 8] = [20, 22, 86, 123, 198, 28, 219, 132];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CollectCreatorFee {
    type ArrangedAccounts = CollectCreatorFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let creator = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let creator_token_0 = next_account(&mut iter)?;
        let creator_token_1 = next_account(&mut iter)?;
        let token_0_program = next_account(&mut iter)?;
        let token_1_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CollectCreatorFeeInstructionAccounts {
            creator,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            creator_token_0,
            creator_token_1,
            token_0_program,
            token_1_program,
            associated_token_program,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
