use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::BondingCurveParam;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePlatformCurveParam {
    pub index: u8,
    pub bonding_curve_param: BondingCurveParam,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePlatformCurveParamInstructionAccounts {
    pub platform_admin: Address,
    pub platform_config: Address,
    pub global_config: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePlatformCurveParam {
    pub const DISCRIMINATOR: [u8; 8] = [138, 144, 138, 250, 220, 128, 4, 57];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePlatformCurveParam {
    type ArrangedAccounts = UpdatePlatformCurveParamInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_admin = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdatePlatformCurveParamInstructionAccounts {
            platform_admin,
            platform_config,
            global_config,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
