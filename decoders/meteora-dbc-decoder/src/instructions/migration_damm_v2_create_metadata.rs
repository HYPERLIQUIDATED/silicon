use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrationDammV2CreateMetadata {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrationDammV2CreateMetadataInstructionAccounts {
    pub virtual_pool: Address,
    pub config: Address,
    pub migration_metadata: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrationDammV2CreateMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [109, 189, 19, 36, 195, 183, 222, 82];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrationDammV2CreateMetadata {
    type ArrangedAccounts = MigrationDammV2CreateMetadataInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrationDammV2CreateMetadataInstructionAccounts {
            virtual_pool,
            config,
            migration_metadata,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
