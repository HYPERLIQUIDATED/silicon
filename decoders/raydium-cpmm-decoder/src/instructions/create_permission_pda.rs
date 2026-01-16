use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePermissionPda {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePermissionPdaInstructionAccounts {
    pub owner: Address,
    pub permission_authority: Address,
    pub permission: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePermissionPda {
    pub const DISCRIMINATOR: [u8; 8] = [135, 136, 2, 216, 137, 169, 181, 202];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePermissionPda {
    type ArrangedAccounts = CreatePermissionPdaInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let permission_authority = next_account(&mut iter)?;
        let permission = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePermissionPdaInstructionAccounts {
            owner,
            permission_authority,
            permission,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
