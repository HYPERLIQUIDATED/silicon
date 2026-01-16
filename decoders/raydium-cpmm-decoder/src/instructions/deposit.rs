use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Deposit {
    pub lp_token_amount: u64,
    pub maximum_token_0_amount: u64,
    pub maximum_token_1_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DepositInstructionAccounts {
    pub owner: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub owner_lp_token: Address,
    pub token_0_account: Address,
    pub token_1_account: Address,
    pub token_0_vault: Address,
    pub token_1_vault: Address,
    pub token_program: Address,
    pub token_program_2022: Address,
    pub vault_0_mint: Address,
    pub vault_1_mint: Address,
    pub lp_mint: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Deposit {
    pub const DISCRIMINATOR: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Deposit {
    type ArrangedAccounts = DepositInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let owner_lp_token = next_account(&mut iter)?;
        let token_0_account = next_account(&mut iter)?;
        let token_1_account = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(DepositInstructionAccounts {
            owner,
            authority,
            pool_state,
            owner_lp_token,
            token_0_account,
            token_1_account,
            token_0_vault,
            token_1_vault,
            token_program,
            token_program_2022,
            vault_0_mint,
            vault_1_mint,
            lp_mint,
            remaining: remaining.to_vec(),
        })
    }
}
