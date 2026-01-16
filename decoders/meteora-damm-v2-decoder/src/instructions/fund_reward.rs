use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FundReward {
    pub reward_index: u8,
    pub amount: u64,
    pub carry_forward: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FundRewardInstructionAccounts {
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

impl FundReward {
    pub const DISCRIMINATOR: [u8; 8] = [188, 50, 249, 165, 93, 151, 38, 63];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for FundReward {
    type ArrangedAccounts = FundRewardInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let funder_token_account = next_account(&mut iter)?;
        let funder = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(FundRewardInstructionAccounts {
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
