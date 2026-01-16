use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateMeteoraDammLockLpToken {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateMeteoraDammLockLpTokenInstructionAccounts {
    pub virtual_pool: Address,
    pub migration_metadata: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub lp_mint: Address,
    pub lock_escrow: Address,
    pub owner: Address,
    pub source_tokens: Address,
    pub escrow_vault: Address,
    pub amm_program: Address,
    pub a_vault: Address,
    pub b_vault: Address,
    pub a_vault_lp: Address,
    pub b_vault_lp: Address,
    pub a_vault_lp_mint: Address,
    pub b_vault_lp_mint: Address,
    pub token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateMeteoraDammLockLpToken {
    pub const DISCRIMINATOR: [u8; 8] = [177, 55, 238, 157, 251, 88, 165, 42];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateMeteoraDammLockLpToken {
    type ArrangedAccounts = MigrateMeteoraDammLockLpTokenInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let lock_escrow = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let source_tokens = next_account(&mut iter)?;
        let escrow_vault = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let a_vault = next_account(&mut iter)?;
        let b_vault = next_account(&mut iter)?;
        let a_vault_lp = next_account(&mut iter)?;
        let b_vault_lp = next_account(&mut iter)?;
        let a_vault_lp_mint = next_account(&mut iter)?;
        let b_vault_lp_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateMeteoraDammLockLpTokenInstructionAccounts {
            virtual_pool,
            migration_metadata,
            pool_authority,
            pool,
            lp_mint,
            lock_escrow,
            owner,
            source_tokens,
            escrow_vault,
            amm_program,
            a_vault,
            b_vault,
            a_vault_lp,
            b_vault_lp,
            a_vault_lp_mint,
            b_vault_lp_mint,
            token_program,
            remaining: remaining.to_vec(),
        })
    }
}
