use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RefreshVesting {}

#[derive(Debug, Clone, PartialEq)]
pub struct RefreshVestingInstructionAccounts {
    pub pool: Address,
    pub position: Address,
    pub position_nft_account: Address,
    pub owner: Address,
    pub remaining: Vec<AccountMeta>,
}

impl RefreshVesting {
    pub const DISCRIMINATOR: [u8; 8] = [9, 94, 216, 14, 116, 204, 247, 0];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for RefreshVesting {
    type ArrangedAccounts = RefreshVestingInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(RefreshVestingInstructionAccounts {
            pool,
            position,
            position_nft_account,
            owner,
            remaining: remaining.to_vec(),
        })
    }
}
