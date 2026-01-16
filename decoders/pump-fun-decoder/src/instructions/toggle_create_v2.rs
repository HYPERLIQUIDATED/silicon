use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ToggleCreateV2 {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleCreateV2InstructionAccounts {
    pub global: Address,
    pub authority: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ToggleCreateV2 {
    pub const DISCRIMINATOR: [u8; 8] = [28, 255, 230, 240, 172, 107, 203, 171];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ToggleCreateV2 {
    type ArrangedAccounts = ToggleCreateV2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ToggleCreateV2InstructionAccounts {
            global,
            authority,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
