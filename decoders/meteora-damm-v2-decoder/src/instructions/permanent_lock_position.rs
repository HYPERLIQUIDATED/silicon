use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PermanentLockPosition {
    pub permanent_lock_liquidity: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PermanentLockPositionInstructionAccounts {
    pub pool: Address,
    pub position: Address,
    pub position_nft_account: Address,
    pub owner: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl PermanentLockPosition {
    pub const DISCRIMINATOR: [u8; 8] = [165, 176, 125, 6, 231, 171, 186, 213];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for PermanentLockPosition {
    type ArrangedAccounts = PermanentLockPositionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(PermanentLockPositionInstructionAccounts {
            pool,
            position,
            position_nft_account,
            owner,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
