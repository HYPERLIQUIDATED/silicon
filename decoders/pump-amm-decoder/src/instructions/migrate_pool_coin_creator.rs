use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigratePoolCoinCreator {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigratePoolCoinCreatorInstructionAccounts {
    pub pool: Address,
    pub sharing_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigratePoolCoinCreator {
    pub const DISCRIMINATOR: [u8; 8] = [208, 8, 159, 4, 74, 175, 16, 58];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigratePoolCoinCreator {
    type ArrangedAccounts = MigratePoolCoinCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let sharing_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigratePoolCoinCreatorInstructionAccounts {
            pool,
            sharing_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
