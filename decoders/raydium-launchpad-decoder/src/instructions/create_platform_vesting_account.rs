use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePlatformVestingAccount {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePlatformVestingAccountInstructionAccounts {
    pub platform_vesting_wallet: Address,
    pub beneficiary: Address,
    pub platform_config: Address,
    pub pool_state: Address,
    pub platform_vesting_record: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePlatformVestingAccount {
    pub const DISCRIMINATOR: [u8; 8] = [146, 71, 173, 69, 98, 19, 15, 106];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePlatformVestingAccount {
    type ArrangedAccounts = CreatePlatformVestingAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_vesting_wallet = next_account(&mut iter)?;
        let beneficiary = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let platform_vesting_record = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePlatformVestingAccountInstructionAccounts {
            platform_vesting_wallet,
            beneficiary,
            platform_config,
            pool_state,
            platform_vesting_record,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
