use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectCreatorFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectCreatorFeeInstructionAccounts {
    pub creator: Address,
    pub creator_vault: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CollectCreatorFee {
    pub const DISCRIMINATOR: [u8; 8] = [20, 22, 86, 123, 198, 28, 219, 132];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CollectCreatorFee {
    type ArrangedAccounts = CollectCreatorFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let creator = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CollectCreatorFeeInstructionAccounts {
            creator,
            creator_vault,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
