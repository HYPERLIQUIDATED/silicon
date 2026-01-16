use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminUpdateTokenIncentives {
    pub start_time: i64,
    pub end_time: i64,
    pub seconds_in_a_day: i64,
    pub day_number: u64,
    pub pump_token_supply_per_day: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdminUpdateTokenIncentivesInstructionAccounts {
    pub authority: Address,
    pub global: Address,
    pub global_volume_accumulator: Address,
    pub mint: Address,
    pub global_incentive_token_account: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl AdminUpdateTokenIncentives {
    pub const DISCRIMINATOR: [u8; 8] = [209, 11, 115, 87, 213, 23, 124, 204];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for AdminUpdateTokenIncentives {
    type ArrangedAccounts = AdminUpdateTokenIncentivesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let global_incentive_token_account = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(AdminUpdateTokenIncentivesInstructionAccounts {
            authority,
            global,
            global_volume_accumulator,
            mint,
            global_incentive_token_account,
            associated_token_program,
            system_program,
            token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
