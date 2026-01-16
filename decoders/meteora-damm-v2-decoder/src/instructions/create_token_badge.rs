use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateTokenBadge {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTokenBadgeInstructionAccounts {
    pub token_badge: Address,
    pub token_mint: Address,
    pub operator: Address,
    pub signer: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateTokenBadge {
    pub const DISCRIMINATOR: [u8; 8] = [88, 206, 0, 91, 60, 175, 151, 118];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateTokenBadge {
    type ArrangedAccounts = CreateTokenBadgeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let token_badge = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateTokenBadgeInstructionAccounts {
            token_badge,
            token_mint,
            operator,
            signer,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
