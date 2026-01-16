use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateMeteoraDamm {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateMeteoraDammInstructionAccounts {
    pub virtual_pool: Address,
    pub migration_metadata: Address,
    pub config: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub damm_config: Address,
    pub lp_mint: Address,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub a_vault: Address,
    pub b_vault: Address,
    pub a_token_vault: Address,
    pub b_token_vault: Address,
    pub a_vault_lp_mint: Address,
    pub b_vault_lp_mint: Address,
    pub a_vault_lp: Address,
    pub b_vault_lp: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub virtual_pool_lp: Address,
    pub protocol_token_a_fee: Address,
    pub protocol_token_b_fee: Address,
    pub payer: Address,
    pub rent: Address,
    pub mint_metadata: Address,
    pub metadata_program: Address,
    pub amm_program: Address,
    pub vault_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateMeteoraDamm {
    pub const DISCRIMINATOR: [u8; 8] = [27, 1, 48, 22, 180, 63, 118, 217];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateMeteoraDamm {
    type ArrangedAccounts = MigrateMeteoraDammInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let damm_config = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let a_vault = next_account(&mut iter)?;
        let b_vault = next_account(&mut iter)?;
        let a_token_vault = next_account(&mut iter)?;
        let b_token_vault = next_account(&mut iter)?;
        let a_vault_lp_mint = next_account(&mut iter)?;
        let b_vault_lp_mint = next_account(&mut iter)?;
        let a_vault_lp = next_account(&mut iter)?;
        let b_vault_lp = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let virtual_pool_lp = next_account(&mut iter)?;
        let protocol_token_a_fee = next_account(&mut iter)?;
        let protocol_token_b_fee = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let mint_metadata = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let vault_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateMeteoraDammInstructionAccounts {
            virtual_pool,
            migration_metadata,
            config,
            pool_authority,
            pool,
            damm_config,
            lp_mint,
            token_a_mint,
            token_b_mint,
            a_vault,
            b_vault,
            a_token_vault,
            b_token_vault,
            a_vault_lp_mint,
            b_vault_lp_mint,
            a_vault_lp,
            b_vault_lp,
            base_vault,
            quote_vault,
            virtual_pool_lp,
            protocol_token_a_fee,
            protocol_token_b_fee,
            payer,
            rent,
            mint_metadata,
            metadata_program,
            amm_program,
            vault_program,
            token_program,
            associated_token_program,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
