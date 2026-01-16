use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CloseUserVolumeAccumulator {}

#[derive(Debug, Clone, PartialEq)]
pub struct CloseUserVolumeAccumulatorInstructionAccounts {
    pub user: Address,
    pub user_volume_accumulator: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CloseUserVolumeAccumulator {
    pub const DISCRIMINATOR: [u8; 8] = [249, 69, 164, 218, 150, 103, 84, 138];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CloseUserVolumeAccumulator {
    type ArrangedAccounts = CloseUserVolumeAccumulatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let user = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CloseUserVolumeAccumulatorInstructionAccounts {
            user,
            user_volume_accumulator,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
