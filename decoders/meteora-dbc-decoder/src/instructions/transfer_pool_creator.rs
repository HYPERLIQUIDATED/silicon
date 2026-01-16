use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TransferPoolCreator {}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferPoolCreatorInstructionAccounts {
    pub virtual_pool: Address,
    pub config: Address,
    pub creator: Address,
    pub new_creator: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl TransferPoolCreator {
    pub const DISCRIMINATOR: [u8; 8] = [20, 7, 169, 33, 58, 147, 166, 33];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for TransferPoolCreator {
    type ArrangedAccounts = TransferPoolCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let new_creator = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(TransferPoolCreatorInstructionAccounts {
            virtual_pool,
            config,
            creator,
            new_creator,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
