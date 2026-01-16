use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RemovePlatformCurveParam {
    pub index: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemovePlatformCurveParamInstructionAccounts {
    pub platform_admin: Address,
    pub platform_config: Address,
    pub remaining: Vec<AccountMeta>,
}

impl RemovePlatformCurveParam {
    pub const DISCRIMINATOR: [u8; 8] = [27, 30, 62, 169, 93, 224, 24, 145];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for RemovePlatformCurveParam {
    type ArrangedAccounts = RemovePlatformCurveParamInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(RemovePlatformCurveParamInstructionAccounts {
            platform_admin,
            platform_config,
            remaining: remaining.to_vec(),
        })
    }
}
