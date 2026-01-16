use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Migrate {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateInstructionAccounts {
    pub global: Address,
    pub withdraw_authority: Address,
    pub mint: Address,
    pub bonding_curve: Address,
    pub associated_bonding_curve: Address,
    pub user: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub pump_amm: Address,
    pub pool: Address,
    pub pool_authority: Address,
    pub pool_authority_mint_account: Address,
    pub pool_authority_wsol_account: Address,
    pub amm_global_config: Address,
    pub wsol_mint: Address,
    pub lp_mint: Address,
    pub user_pool_token_account: Address,
    pub pool_base_token_account: Address,
    pub pool_quote_token_account: Address,
    pub token2022_program: Address,
    pub associated_token_program: Address,
    pub pump_amm_event_authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Migrate {
    pub const DISCRIMINATOR: [u8; 8] = [155, 234, 231, 146, 236, 158, 162, 30];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Migrate {
    type ArrangedAccounts = MigrateInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let withdraw_authority = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let pump_amm = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool_authority_mint_account = next_account(&mut iter)?;
        let pool_authority_wsol_account = next_account(&mut iter)?;
        let amm_global_config = next_account(&mut iter)?;
        let wsol_mint = next_account(&mut iter)?;
        let lp_mint = next_account(&mut iter)?;
        let user_pool_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let pump_amm_event_authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateInstructionAccounts {
            global,
            withdraw_authority,
            mint,
            bonding_curve,
            associated_bonding_curve,
            user,
            system_program,
            token_program,
            pump_amm,
            pool,
            pool_authority,
            pool_authority_mint_account,
            pool_authority_wsol_account,
            amm_global_config,
            wsol_mint,
            lp_mint,
            user_pool_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            token2022_program,
            associated_token_program,
            pump_amm_event_authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
