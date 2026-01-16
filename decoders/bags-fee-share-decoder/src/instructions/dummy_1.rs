use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::FeeShareConfig;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Dummy1 {
    pub input: FeeShareConfig,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dummy1InstructionAccounts {
    pub signer: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Dummy1 {
    pub const DISCRIMINATOR: [u8; 8] = [159, 255, 61, 156, 197, 198, 94, 200];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Dummy1 {
    type ArrangedAccounts = Dummy1InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let signer = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(Dummy1InstructionAccounts {
            signer,
            remaining: remaining.to_vec(),
        })
    }
}
