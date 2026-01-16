use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimA {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimAInstructionAccounts {
    pub claimer_a: Address,
    pub claimer_b: Address,
    pub fee_authority: Address,
    pub fee_authority_quote_ata: Address,
    pub fee_authority_base_ata: Address,
    pub vault_a: Address,
    pub vault_b: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub associated_token_program: Address,
    pub dbc_program: Address,
    pub pool_authority: Address,
    pub config: Address,
    pub pool: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub dbc_event_authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimA {
    pub const DISCRIMINATOR: [u8; 8] = [161, 169, 182, 105, 63, 187, 190, 46];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimA {
    type ArrangedAccounts = ClaimAInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let claimer_a = next_account(&mut iter)?;
        let claimer_b = next_account(&mut iter)?;
        let fee_authority = next_account(&mut iter)?;
        let fee_authority_quote_ata = next_account(&mut iter)?;
        let fee_authority_base_ata = next_account(&mut iter)?;
        let vault_a = next_account(&mut iter)?;
        let vault_b = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let dbc_program = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let dbc_event_authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimAInstructionAccounts {
            claimer_a,
            claimer_b,
            fee_authority,
            fee_authority_quote_ata,
            fee_authority_base_ata,
            vault_a,
            vault_b,
            base_mint,
            quote_mint,
            system_program,
            token_base_program,
            token_quote_program,
            associated_token_program,
            dbc_program,
            pool_authority,
            config,
            pool,
            base_vault,
            quote_vault,
            dbc_event_authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
