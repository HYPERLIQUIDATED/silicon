use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePoolStatus {
    pub status: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePoolStatusInstructionAccounts {
    pub authority: Address,
    pub pool_state: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePoolStatus {
    pub const DISCRIMINATOR: [u8; 8] = [130, 87, 108, 6, 46, 224, 117, 123];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePoolStatus {
    type ArrangedAccounts = UpdatePoolStatusInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdatePoolStatusInstructionAccounts {
            authority,
            pool_state,
            remaining: remaining.to_vec(),
        })
    }
}
