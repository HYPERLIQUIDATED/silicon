use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct WithdrawIneligibleReward {
    pub reward_index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawIneligibleRewardInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub reward_vault: Address,
    pub reward_mint: Address,
    pub funder_token_account: Address,
    pub funder: Address,
    pub token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl WithdrawIneligibleReward {
    pub const DISCRIMINATOR: [u8; 8] = [148, 206, 42, 195, 247, 49, 103, 8];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for WithdrawIneligibleReward {
    type ArrangedAccounts = WithdrawIneligibleRewardInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(WithdrawIneligibleRewardInstructionAccounts {
            pool_authority,
            pool,
            reward_vault,
            reward_mint,
            funder_token_account,
            funder,
            token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
