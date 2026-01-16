use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::UpdatePartnerConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdatePartnerConfig {
    pub params: UpdatePartnerConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePartnerConfigInstructionAccounts {
    pub admin: Address,
    pub program_config: Address,
    pub partner_config: Address,
    pub partner: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdatePartnerConfig {
    pub const DISCRIMINATOR: [u8; 8] = [111, 77, 242, 174, 244, 48, 138, 213];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdatePartnerConfig {
    type ArrangedAccounts = UpdatePartnerConfigInstructionAccounts;

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

        Some(UpdatePartnerConfigInstructionAccounts {
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
