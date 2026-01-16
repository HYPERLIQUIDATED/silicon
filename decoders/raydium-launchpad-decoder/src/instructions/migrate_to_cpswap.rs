use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateToCpswap {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateToCpswapInstructionAccounts {
    pub payer: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub platform_config: Address,
    pub cpswap_program: Address,
    pub cpswap_pool: Address,
    pub cpswap_authority: Address,
    pub cpswap_lp_mint: Address,
    pub cpswap_base_vault: Address,
    pub cpswap_quote_vault: Address,
    pub cpswap_config: Address,
    pub cpswap_create_pool_fee: Address,
    pub cpswap_observation: Address,
    pub lock_program: Address,
    pub lock_authority: Address,
    pub lock_lp_vault: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub global_config: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub pool_lp_token: Address,
    pub base_token_program: Address,
    pub quote_token_program: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub rent_program: Address,
    pub metadata_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateToCpswap {
    pub const DISCRIMINATOR: [u8; 8] = [136, 92, 200, 103, 28, 218, 144, 140];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateToCpswap {
    type ArrangedAccounts = MigrateToCpswapInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let cpswap_program = next_account(&mut iter)?;
        let cpswap_pool = next_account(&mut iter)?;
        let cpswap_authority = next_account(&mut iter)?;
        let cpswap_lp_mint = next_account(&mut iter)?;
        let cpswap_base_vault = next_account(&mut iter)?;
        let cpswap_quote_vault = next_account(&mut iter)?;
        let cpswap_config = next_account(&mut iter)?;
        let cpswap_create_pool_fee = next_account(&mut iter)?;
        let cpswap_observation = next_account(&mut iter)?;
        let lock_program = next_account(&mut iter)?;
        let lock_authority = next_account(&mut iter)?;
        let lock_lp_vault = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let pool_lp_token = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateToCpswapInstructionAccounts {
            payer,
            base_mint,
            quote_mint,
            platform_config,
            cpswap_program,
            cpswap_pool,
            cpswap_authority,
            cpswap_lp_mint,
            cpswap_base_vault,
            cpswap_quote_vault,
            cpswap_config,
            cpswap_create_pool_fee,
            cpswap_observation,
            lock_program,
            lock_authority,
            lock_lp_vault,
            authority,
            pool_state,
            global_config,
            base_vault,
            quote_vault,
            pool_lp_token,
            base_token_program,
            quote_token_program,
            associated_token_program,
            system_program,
            rent_program,
            metadata_program,
            remaining: remaining.to_vec(),
        })
    }
}
