use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::UpdateFeeConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateFeeConfig {
    pub params: UpdateFeeConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateFeeConfigInstructionAccounts {
    pub admin: Address,
    pub program_config: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateFeeConfig {
    pub const DISCRIMINATOR: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateFeeConfig {
    type ArrangedAccounts = UpdateFeeConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let fee_share_config = next_account(&mut iter)?;
        let fee_share_authority = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateFeeConfigInstructionAccounts {
            admin,
            program_config,
            fee_share_config,
            fee_share_authority,
            base_mint,
            quote_mint,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
