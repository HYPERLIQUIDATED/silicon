use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CloseOperatorAccount {}

#[derive(Debug, Clone, PartialEq)]
pub struct CloseOperatorAccountInstructionAccounts {
    pub operator: Address,
    pub signer: Address,
    pub rent_receiver: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CloseOperatorAccount {
    pub const DISCRIMINATOR: [u8; 8] = [171, 9, 213, 74, 120, 23, 3, 29];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CloseOperatorAccount {
    type ArrangedAccounts = CloseOperatorAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CloseOperatorAccountInstructionAccounts {
            operator,
            signer,
            rent_receiver,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
