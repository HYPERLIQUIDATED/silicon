use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitUserVolumeAccumulator {}

#[derive(Debug, Clone, PartialEq)]
pub struct InitUserVolumeAccumulatorInstructionAccounts {
    pub payer: Address,
    pub user: Address,
    pub user_volume_accumulator: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitUserVolumeAccumulator {
    pub const DISCRIMINATOR: [u8; 8] = [94, 6, 202, 115, 255, 96, 232, 183];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitUserVolumeAccumulator {
    type ArrangedAccounts = InitUserVolumeAccumulatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitUserVolumeAccumulatorInstructionAccounts {
            payer,
            user,
            user_volume_accumulator,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
