use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::CreatorFeeOn;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeWithPermission {
    pub init_amount_0: u64,
    pub init_amount_1: u64,
    pub open_time: u64,
    pub creator_fee_on: CreatorFeeOn,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeWithPermissionInstructionAccounts {
    pub payer: Address,
    pub creator: Address,
    pub amm_config: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub token_0_mint: Address,
    pub token_1_mint: Address,
    pub lp_mint: Address,
    pub payer_token_0: Address,
    pub payer_token_1: Address,
    pub payer_lp_token: Address,
    pub token_0_vault: Address,
    pub token_1_vault: Address,
    pub create_pool_fee: Address,
    pub observation_state: Address,
    pub permission: Address,
    pub token_program: Address,
    pub token_0_program: Address,
    pub token_1_program: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeWithPermission {
    pub const DISCRIMINATOR: [u8; 8] = [63, 55, 254, 65, 49, 178, 89, 121];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeWithPermission {
    type ArrangedAccounts = InitializeWithPermissionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let token_0_mint = next_account(&mut iter)?;
        let token_1_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let payer_token_0 = next_account(&mut iter)?;
        let payer_token_1 = next_account(&mut iter)?;
        let payer_lp_token = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let create_pool_fee = next_account(&mut iter)?;
        let observation_state = next_account(&mut iter)?;
        let permission = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_0_program = next_account(&mut iter)?;
        let token_1_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeWithPermissionInstructionAccounts {
            payer,
            creator,
            amm_config,
            authority,
            pool_state,
            token_0_mint,
            token_1_mint,
            lp_mint,
            payer_token_0,
            payer_token_1,
            payer_lp_token,
            token_0_vault,
            token_1_vault,
            create_pool_fee,
            observation_state,
            permission,
            token_program,
            token_0_program,
            token_1_program,
            associated_token_program,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
