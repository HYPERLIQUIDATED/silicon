use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SyncUserVolumeAccumulator {}

#[derive(Debug, Clone, PartialEq)]
pub struct SyncUserVolumeAccumulatorInstructionAccounts {
    pub user: Address,
    pub global_volume_accumulator: Address,
    pub user_volume_accumulator: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SyncUserVolumeAccumulator {
    pub const DISCRIMINATOR: [u8; 8] = [86, 31, 192, 87, 163, 87, 79, 238];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SyncUserVolumeAccumulator {
    type ArrangedAccounts = SyncUserVolumeAccumulatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let user = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SyncUserVolumeAccumulatorInstructionAccounts {
            user,
            global_volume_accumulator,
            user_volume_accumulator,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
