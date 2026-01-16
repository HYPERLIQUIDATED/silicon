use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimTokenIncentives {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimTokenIncentivesInstructionAccounts {
    pub user: Address,
    pub user_ata: Address,
    pub global_volume_accumulator: Address,
    pub global_incentive_token_account: Address,
    pub user_volume_accumulator: Address,
    pub mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub payer: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimTokenIncentives {
    pub const DISCRIMINATOR: [u8; 8] = [16, 4, 71, 28, 204, 1, 40, 27];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimTokenIncentives {
    type ArrangedAccounts = ClaimTokenIncentivesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let user = next_account(&mut iter)?;
        let user_ata = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let global_incentive_token_account = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimTokenIncentivesInstructionAccounts {
            user,
            user_ata,
            global_volume_accumulator,
            global_incentive_token_account,
            user_volume_accumulator,
            mint,
            token_program,
            system_program,
            associated_token_program,
            event_authority,
            program,
            payer,
            remaining: remaining.to_vec(),
        })
    }
}
