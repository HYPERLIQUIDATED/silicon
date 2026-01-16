use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateOperatorAccount {
    pub permission: u128,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateOperatorAccountInstructionAccounts {
    pub operator: Address,
    pub whitelisted_address: Address,
    pub signer: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateOperatorAccount {
    pub const DISCRIMINATOR: [u8; 8] = [221, 64, 246, 149, 240, 153, 229, 163];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateOperatorAccount {
    type ArrangedAccounts = CreateOperatorAccountInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let operator = next_account(&mut iter)?;
        let whitelisted_address = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateOperatorAccountInstructionAccounts {
            operator,
            whitelisted_address,
            signer,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
