use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateMeteoraDammClaimLpToken {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateMeteoraDammClaimLpTokenInstructionAccounts {
    pub virtual_pool: Address,
    pub migration_metadata: Address,
    pub pool_authority: Address,
    pub lp_mint: Address,
    pub source_token: Address,
    pub destination_token: Address,
    pub owner: Address,
    pub sender: Address,
    pub token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateMeteoraDammClaimLpToken {
    pub const DISCRIMINATOR: [u8; 8] = [139, 133, 2, 30, 91, 145, 127, 154];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateMeteoraDammClaimLpToken {
    type ArrangedAccounts = MigrateMeteoraDammClaimLpTokenInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let migration_metadata = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let source_token = next_account(&mut iter)?;
        let destination_token = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let sender = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateMeteoraDammClaimLpTokenInstructionAccounts {
            virtual_pool,
            migration_metadata,
            pool_authority,
            lp_mint,
            source_token,
            destination_token,
            owner,
            sender,
            token_program,
            remaining: remaining.to_vec(),
        })
    }
}
