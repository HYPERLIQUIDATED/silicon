use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::BaseFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FixConfigFeeParams {
    pub params: BaseFeeParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixConfigFeeParamsInstructionAccounts {
    pub config: Address,
    pub operator: Address,
    pub signer: Address,
    pub remaining: Vec<AccountMeta>,
}

impl FixConfigFeeParams {
    pub const DISCRIMINATOR: [u8; 8] = [38, 30, 216, 81, 250, 177, 243, 254];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for FixConfigFeeParams {
    type ArrangedAccounts = FixConfigFeeParamsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let config = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(FixConfigFeeParamsInstructionAccounts {
            config,
            operator,
            signer,
            remaining: remaining.to_vec(),
        })
    }
}
