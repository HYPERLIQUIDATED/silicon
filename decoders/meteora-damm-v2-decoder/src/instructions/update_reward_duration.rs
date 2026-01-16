use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateRewardDuration {
    pub reward_index: u8,
    pub new_duration: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateRewardDurationInstructionAccounts {
    pub pool: Address,
    pub signer: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateRewardDuration {
    pub const DISCRIMINATOR: [u8; 8] = [138, 174, 196, 169, 213, 235, 254, 107];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateRewardDuration {
    type ArrangedAccounts = UpdateRewardDurationInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateRewardDurationInstructionAccounts {
            pool,
            signer,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
