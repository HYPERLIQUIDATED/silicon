use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetMetaplexCreator {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetMetaplexCreatorInstructionAccounts {
    pub mint: Address,
    pub metadata: Address,
    pub bonding_curve: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetMetaplexCreator {
    pub const DISCRIMINATOR: [u8; 8] = [138, 96, 174, 217, 48, 85, 197, 246];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetMetaplexCreator {
    type ArrangedAccounts = SetMetaplexCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetMetaplexCreatorInstructionAccounts {
            mint,
            metadata,
            bonding_curve,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
