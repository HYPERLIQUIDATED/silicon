use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateConfig {
    pub param: u8,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateConfigInstructionAccounts {
    pub owner: Address,
    pub global_config: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateConfig {
    pub const DISCRIMINATOR: [u8; 8] = [29, 158, 252, 191, 10, 83, 219, 99];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateConfig {
    type ArrangedAccounts = UpdateConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateConfigInstructionAccounts {
            owner,
            global_config,
            remaining: remaining.to_vec(),
        })
    }
}
