use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::{AmmCreatorFeeOn, CurveParams, MintParams, VestingParams};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitializeV2 {
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InitializeV2InstructionAccounts {
    pub payer: Address,
    pub creator: Address,
    pub global_config: Address,
    pub platform_config: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub metadata_account: Address,
    pub base_token_program: Address,
    pub quote_token_program: Address,
    pub metadata_program: Address,
    pub system_program: Address,
    pub rent_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitializeV2 {
    pub const DISCRIMINATOR: [u8; 8] = [67, 153, 175, 39, 218, 16, 38, 32];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitializeV2 {
    type ArrangedAccounts = InitializeV2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let metadata_account = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let metadata_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitializeV2InstructionAccounts {
            payer,
            creator,
            global_config,
            platform_config,
            authority,
            pool_state,
            base_mint,
            quote_mint,
            base_vault,
            quote_vault,
            metadata_account,
            base_token_program,
            quote_token_program,
            metadata_program,
            system_program,
            rent_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
