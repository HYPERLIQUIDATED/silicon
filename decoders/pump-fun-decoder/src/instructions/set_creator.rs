use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetCreator {
    pub creator: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetCreatorInstructionAccounts {
    pub set_creator_authority: Address,
    pub global: Address,
    pub mint: Address,
    pub metadata: Address,
    pub bonding_curve: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetCreator {
    pub const DISCRIMINATOR: [u8; 8] = [254, 148, 255, 112, 207, 142, 170, 165];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetCreator {
    type ArrangedAccounts = SetCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let set_creator_authority = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetCreatorInstructionAccounts {
            set_creator_authority,
            global,
            mint,
            metadata,
            bonding_curve,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
