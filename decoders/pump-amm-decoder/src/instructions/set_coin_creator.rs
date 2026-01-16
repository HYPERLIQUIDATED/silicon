use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetCoinCreator {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCoinCreatorInstructionAccounts {
    pub pool: Address,
    pub metadata: Address,
    pub bonding_curve: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetCoinCreator {
    pub const DISCRIMINATOR: [u8; 8] = [210, 149, 128, 45, 188, 58, 78, 175];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetCoinCreator {
    type ArrangedAccounts = SetCoinCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetCoinCreatorInstructionAccounts {
            pool,
            metadata,
            bonding_curve,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
