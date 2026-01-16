use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetCreator {
    pub creator: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdminSetCreatorInstructionAccounts {
    pub admin_set_creator_authority: Address,
    pub global: Address,
    pub mint: Address,
    pub bonding_curve: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl AdminSetCreator {
    pub const DISCRIMINATOR: [u8; 8] = [69, 25, 171, 142, 57, 239, 13, 4];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for AdminSetCreator {
    type ArrangedAccounts = AdminSetCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin_set_creator_authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(AdminSetCreatorInstructionAccounts {
            admin_set_creator_authority,
            global,
            mint,
            bonding_curve,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
