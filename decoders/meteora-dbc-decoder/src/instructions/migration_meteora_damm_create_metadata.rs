use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrationMeteoraDammCreateMetadata {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrationMeteoraDammCreateMetadataInstructionAccounts {
    pub virtual_pool: Address,
    pub config: Address,
    pub migration_metadata: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrationMeteoraDammCreateMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [47, 94, 126, 115, 221, 226, 194, 133];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrationMeteoraDammCreateMetadata {
    type ArrangedAccounts = MigrationMeteoraDammCreateMetadataInstructionAccounts;

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

        Some(MigrationMeteoraDammCreateMetadataInstructionAccounts {
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
