use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimReward {
    pub reward_index: u8,
    pub skip_reward: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimRewardInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub position: Address,
    pub reward_vault: Address,
    pub reward_mint: Address,
    pub user_token_account: Address,
    pub position_nft_account: Address,
    pub owner: Address,
    pub token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimReward {
    pub const DISCRIMINATOR: [u8; 8] = [149, 95, 181, 242, 94, 90, 158, 162];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimReward {
    type ArrangedAccounts = ClaimRewardInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let user_token_account = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimRewardInstructionAccounts {
            pool_authority,
            pool,
            position,
            reward_vault,
            reward_mint,
            user_token_account,
            position_nft_account,
            owner,
            token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
