use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClosePermissionPda {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosePermissionPdaInstructionAccounts {
    pub owner: Address,
    pub permission_authority: Address,
    pub permission: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClosePermissionPda {
    pub const DISCRIMINATOR: [u8; 8] = [156, 84, 32, 118, 69, 135, 70, 123];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClosePermissionPda {
    type ArrangedAccounts = ClosePermissionPdaInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let permission_authority = next_account(&mut iter)?;
        let permission = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClosePermissionPdaInstructionAccounts {
            owner,
            permission_authority,
            permission,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
