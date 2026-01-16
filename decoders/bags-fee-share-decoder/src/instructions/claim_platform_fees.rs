use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimPlatformFees {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimPlatformFeesInstructionAccounts {
    pub admin: Address,
    pub receiver: Address,
    pub program_config: Address,
    pub platform_vault: Address,
    pub receiver_quote_ata: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimPlatformFees {
    pub const DISCRIMINATOR: [u8; 8] = [159, 129, 37, 35, 170, 99, 163, 16];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimPlatformFees {
    type ArrangedAccounts = ClaimPlatformFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let receiver = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let platform_vault = next_account(&mut iter)?;
        let receiver_quote_ata = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimPlatformFeesInstructionAccounts {
            admin,
            receiver,
            program_config,
            platform_vault,
            receiver_quote_ata,
            quote_mint,
            system_program,
            token_program,
            associated_token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
