use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeReward {
    pub reward_index: u8,
    pub reward_duration: u64,
    pub funder: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeRewardInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub reward_vault: Address,
    pub reward_mint: Address,
    pub signer: Address,
    pub payer: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeReward {
    pub const DISCRIMINATOR: [u8; 8] = [95, 135, 192, 196, 242, 129, 230, 68];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let reward_vault = next_account(&mut iter)?;
        let reward_mint = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeRewardInstructionAccounts {
            pool_authority,
            pool,
            reward_vault,
            reward_mint,
            signer,
            payer,
            token_program,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
