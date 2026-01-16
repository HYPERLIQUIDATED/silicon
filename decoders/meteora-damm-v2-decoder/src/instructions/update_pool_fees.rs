use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::UpdatePoolFeesParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePoolFees {
    pub params: UpdatePoolFeesParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePoolFeesInstructionAccounts {
    pub pool: Address,
    pub operator: Address,
    pub signer: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePoolFees {
    pub const DISCRIMINATOR: [u8; 8] = [118, 217, 203, 179, 60, 8, 70, 89];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePoolFees {
    type ArrangedAccounts = UpdatePoolFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdatePoolFeesInstructionAccounts {
            pool,
            operator,
            signer,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
