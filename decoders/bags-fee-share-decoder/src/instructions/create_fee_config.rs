use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::CreateFeeConfigParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateFeeConfig {
    pub params: CreateFeeConfigParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateFeeConfigInstructionAccounts {
    pub payer: Address,
    pub admin: Address,
    pub program_config: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub fee_share_authority_quote_ata: Address,
    pub partner_config: Option<Address>,
    pub partner: Option<Address>,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateFeeConfig {
    pub const DISCRIMINATOR: [u8; 8] = [214, 172, 105, 64, 8, 228, 209, 204];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateFeeConfig {
    type ArrangedAccounts = CreateFeeConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let fee_share_config = next_account(&mut iter)?;
        let fee_share_authority = next_account(&mut iter)?;
        let fee_share_authority_quote_ata = next_account(&mut iter)?;
        let partner_config = next_account(&mut iter);
        let partner = next_account(&mut iter);
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateFeeConfigInstructionAccounts {
            payer,
            admin,
            program_config,
            fee_share_config,
            fee_share_authority,
            fee_share_authority_quote_ata,
            partner_config,
            partner,
            base_mint,
            quote_mint,
            system_program,
            token_program,
            associated_token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
