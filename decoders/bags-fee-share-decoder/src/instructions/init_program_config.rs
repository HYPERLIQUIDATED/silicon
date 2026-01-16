use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct InitProgramConfig {}

#[derive(Debug, Clone, PartialEq)]
pub struct InitProgramConfigInstructionAccounts {
    pub payer: Address,
    pub admin: Address,
    pub program_config: Address,
    pub platform_vault: Address,
    pub quote_mint: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl InitProgramConfig {
    pub const DISCRIMINATOR: [u8; 8] = [185, 54, 237, 229, 219, 179, 109, 20];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for InitProgramConfig {
    type ArrangedAccounts = InitProgramConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let admin = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let platform_vault = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(InitProgramConfigInstructionAccounts {
            payer,
            admin,
            program_config,
            platform_vault,
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
