use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::CreatePartnerMetadataParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePartnerMetadata {
    pub metadata: CreatePartnerMetadataParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePartnerMetadataInstructionAccounts {
    pub partner_metadata: Address,
    pub payer: Address,
    pub fee_claimer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePartnerMetadata {
    pub const DISCRIMINATOR: [u8; 8] = [192, 168, 234, 191, 188, 226, 227, 255];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePartnerMetadata {
    type ArrangedAccounts = CreatePartnerMetadataInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let partner_metadata = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let fee_claimer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePartnerMetadataInstructionAccounts {
            partner_metadata,
            payer,
            fee_claimer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
