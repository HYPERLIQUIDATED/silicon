use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::VestingParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LockPosition {
    pub params: VestingParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LockPositionInstructionAccounts {
    pub pool: Address,
    pub position: Address,
    pub vesting: Address,
    pub position_nft_account: Address,
    pub owner: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl LockPosition {
    pub const DISCRIMINATOR: [u8; 8] = [227, 62, 2, 252, 247, 10, 171, 185];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for LockPosition {
    type ArrangedAccounts = LockPositionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let vesting = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(LockPositionInstructionAccounts {
            pool,
            position,
            vesting,
            position_nft_account,
            owner,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
