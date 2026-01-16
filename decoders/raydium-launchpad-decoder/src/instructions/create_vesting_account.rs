use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateVestingAccount {
    pub share_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateVestingAccountInstructionAccounts {
    pub creator: Address,
    pub beneficiary: Address,
    pub pool_state: Address,
    pub vesting_record: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateVestingAccount {
    pub const DISCRIMINATOR: [u8; 8] = [129, 178, 2, 13, 217, 172, 230, 218];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateVestingAccount {
    type ArrangedAccounts = CreateVestingAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let creator = next_account(&mut iter)?;
        let beneficiary = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let vesting_record = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateVestingAccountInstructionAccounts {
            creator,
            beneficiary,
            pool_state,
            vesting_record,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
