use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::FeeShareAuthority;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Dummy2 {
    pub input: FeeShareAuthority,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dummy2InstructionAccounts {
    pub signer: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Dummy2 {
    pub const DISCRIMINATOR: [u8; 8] = [156, 133, 43, 254, 54, 30, 107, 7];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Dummy2 {
    type ArrangedAccounts = Dummy2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let signer = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(Dummy2InstructionAccounts {
            signer,
            remaining: remaining.to_vec(),
        })
    }
}
