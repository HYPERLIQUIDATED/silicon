use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::CreateVirtualPoolMetadataParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateVirtualPoolMetadata {
    pub metadata: CreateVirtualPoolMetadataParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateVirtualPoolMetadataInstructionAccounts {
    pub virtual_pool: Address,
    pub virtual_pool_metadata: Address,
    pub creator: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateVirtualPoolMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [45, 97, 187, 103, 254, 109, 124, 134];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateVirtualPoolMetadata {
    type ArrangedAccounts = CreateVirtualPoolMetadataInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let virtual_pool_metadata = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateVirtualPoolMetadataInstructionAccounts {
            virtual_pool,
            virtual_pool_metadata,
            creator,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
