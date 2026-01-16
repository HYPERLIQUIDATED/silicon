use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ToggleMayhemMode {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToggleMayhemModeInstructionAccounts {
    pub admin: Address,
    pub global_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ToggleMayhemMode {
    pub const DISCRIMINATOR: [u8; 8] = [1, 9, 111, 208, 100, 31, 255, 163];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ToggleMayhemMode {
    type ArrangedAccounts = ToggleMayhemModeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ToggleMayhemModeInstructionAccounts {
            admin,
            global_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
