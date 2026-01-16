use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SplitPosition2 {
    pub numerator: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SplitPosition2InstructionAccounts {
    pub pool: Address,
    pub first_position: Address,
    pub first_position_nft_account: Address,
    pub second_position: Address,
    pub second_position_nft_account: Address,
    pub first_owner: Address,
    pub second_owner: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SplitPosition2 {
    pub const DISCRIMINATOR: [u8; 8] = [221, 147, 228, 207, 140, 212, 17, 119];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SplitPosition2 {
    type ArrangedAccounts = SplitPosition2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let first_position = next_account(&mut iter)?;
        let first_position_nft_account = next_account(&mut iter)?;
        let second_position = next_account(&mut iter)?;
        let second_position_nft_account = next_account(&mut iter)?;
        let first_owner = next_account(&mut iter)?;
        let second_owner = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SplitPosition2InstructionAccounts {
            pool,
            first_position,
            first_position_nft_account,
            second_position,
            second_position_nft_account,
            first_owner,
            second_owner,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
