use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::PlatformConfigParam;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePlatformConfig {
    pub param: PlatformConfigParam,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePlatformConfigInstructionAccounts {
    pub platform_admin: Address,
    pub platform_config: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePlatformConfig {
    pub const DISCRIMINATOR: [u8; 8] = [195, 60, 76, 129, 146, 45, 67, 143];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePlatformConfig {
    type ArrangedAccounts = UpdatePlatformConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdatePlatformConfigInstructionAccounts {
            platform_admin,
            platform_config,
            remaining: remaining.to_vec(),
        })
    }
}
