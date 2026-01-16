use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::ExtendCreatedFeeConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ExtendCreatedFeeConfig {
    pub params: ExtendCreatedFeeConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtendCreatedFeeConfigInstructionAccounts {
    pub payer: Address,
    pub admin: Address,
    pub program_config: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub partner: Option<Address>,
    pub partner_config: Option<Address>,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ExtendCreatedFeeConfig {
    pub const DISCRIMINATOR: [u8; 8] = [205, 172, 113, 254, 225, 59, 82, 79];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ExtendCreatedFeeConfig {
    type ArrangedAccounts = ExtendCreatedFeeConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let fee_share_config = next_account(&mut iter)?;
        let fee_share_authority = next_account(&mut iter)?;
        let partner = next_account(&mut iter);
        let partner_config = next_account(&mut iter);
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ExtendCreatedFeeConfigInstructionAccounts {
            payer,
            admin,
            program_config,
            fee_share_config,
            fee_share_authority,
            partner,
            partner_config,
            base_mint,
            quote_mint,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
