use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrationDammV2 {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrationDammV2InstructionAccounts {
    pub virtual_pool: Address,
    pub migration_metadata: Address,
    pub config: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub first_position_nft_mint: Address,
    pub first_position_nft_account: Address,
    pub first_position: Address,
    pub second_position_nft_mint: Option<Address>,
    pub second_position_nft_account: Option<Address>,
    pub second_position: Option<Address>,
    pub damm_pool_authority: Address,
    pub amm_program: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub token_a_vault: Address,
    pub token_b_vault: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub payer: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub token2022_program: Address,
    pub damm_event_authority: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrationDammV2 {
    pub const DISCRIMINATOR: [u8; 8] = [156, 169, 230, 103, 53, 228, 80, 64];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrationDammV2 {
    type ArrangedAccounts = MigrationDammV2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let first_position_nft_mint = next_account(&mut iter)?;
        let first_position_nft_account = next_account(&mut iter)?;
        let first_position = next_account(&mut iter)?;
        let second_position_nft_mint = next_account(&mut iter);
        let second_position_nft_account = next_account(&mut iter);
        let second_position = next_account(&mut iter);
        let damm_pool_authority = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let damm_event_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrationDammV2InstructionAccounts {
            virtual_pool,
            migration_metadata,
            config,
            pool_authority,
            pool,
            first_position_nft_mint,
            first_position_nft_account,
            first_position,
            second_position_nft_mint,
            second_position_nft_account,
            second_position,
            damm_pool_authority,
            amm_program,
            base_mint,
            quote_mint,
            token_a_vault,
            token_b_vault,
            base_vault,
            quote_vault,
            payer,
            token_base_program,
            token_quote_program,
            token2022_program,
            damm_event_authority,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
