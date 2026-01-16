use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::UpdatePartnerFeeCollectionParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePartnerFeeCollection {
    pub params: UpdatePartnerFeeCollectionParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePartnerFeeCollectionInstructionAccounts {
    pub admin: Address,
    pub program_config: Address,
    pub partner_config: Address,
    pub partner: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePartnerFeeCollection {
    pub const DISCRIMINATOR: [u8; 8] = [155, 56, 159, 217, 130, 217, 252, 72];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePartnerFeeCollection {
    type ArrangedAccounts = UpdatePartnerFeeCollectionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let partner_config = next_account(&mut iter)?;
        let partner = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdatePartnerFeeCollectionInstructionAccounts {
            admin,
            program_config,
            partner_config,
            partner,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
