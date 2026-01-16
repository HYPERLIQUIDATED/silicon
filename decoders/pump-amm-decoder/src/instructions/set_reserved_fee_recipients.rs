use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetReservedFeeRecipients {
    pub whitelist_pda: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetReservedFeeRecipientsInstructionAccounts {
    pub global_config: Address,
    pub admin: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetReservedFeeRecipients {
    pub const DISCRIMINATOR: [u8; 8] = [111, 172, 162, 232, 114, 89, 213, 142];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetReservedFeeRecipients {
    type ArrangedAccounts = SetReservedFeeRecipientsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global_config = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetReservedFeeRecipientsInstructionAccounts {
            global_config,
            admin,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
