use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::RemoveLiquidityParameters;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct RemoveLiquidity {
    pub params: RemoveLiquidityParameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveLiquidityInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub position: Address,
    pub token_a_account: Address,
    pub token_b_account: Address,
    pub token_a_vault: Address,
    pub token_b_vault: Address,
    pub token_a_mint: Address,
    pub token_b_mint: Address,
    pub position_nft_account: Address,
    pub owner: Address,
    pub token_a_program: Address,
    pub token_b_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl RemoveLiquidity {
    pub const DISCRIMINATOR: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for RemoveLiquidity {
    type ArrangedAccounts = RemoveLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let token_a_account = next_account(&mut iter)?;
        let token_b_account = next_account(&mut iter)?;
        let token_a_vault = next_account(&mut iter)?;
        let token_b_vault = next_account(&mut iter)?;
        let token_a_mint = next_account(&mut iter)?;
        let token_b_mint = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_a_program = next_account(&mut iter)?;
        let token_b_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(RemoveLiquidityInstructionAccounts {
            pool_authority,
            pool,
            position,
            token_a_account,
            token_b_account,
            token_a_vault,
            token_b_vault,
            token_a_mint,
            token_b_mint,
            position_nft_account,
            owner,
            token_a_program,
            token_b_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
