use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::BaseFeeParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FixPoolFeeParams {
    pub params: BaseFeeParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FixPoolFeeParamsInstructionAccounts {
    pub pool: Address,
    pub operator: Address,
    pub signer: Address,
    pub remaining: Vec<AccountMeta>,
}

impl FixPoolFeeParams {
    pub const DISCRIMINATOR: [u8; 8] = [132, 98, 81, 196, 44, 58, 120, 193];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for FixPoolFeeParams {
    type ArrangedAccounts = FixPoolFeeParamsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(FixPoolFeeParamsInstructionAccounts {
            pool,
            operator,
            signer,
            remaining: remaining.to_vec(),
        })
    }
}
