use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimDammB {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimDammBInstructionAccounts {
    pub claimer_b: Address,
    pub claimer_a: Address,
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
    pub damm_program: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub position: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub position_nft_account: Address,
    pub damm_event_authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimDammB {
    pub const DISCRIMINATOR: [u8; 8] = [85, 187, 79, 227, 148, 222, 169, 95];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimDammB {
    type ArrangedAccounts = ClaimDammBInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let claimer_b = next_account(&mut iter)?;
        let claimer_a = next_account(&mut iter)?;
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
        let damm_program = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let damm_event_authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimDammBInstructionAccounts {
            claimer_b,
            claimer_a,
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
            damm_program,
            pool_authority,
            pool,
            position,
            base_vault,
            quote_vault,
            position_nft_account,
            damm_event_authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
