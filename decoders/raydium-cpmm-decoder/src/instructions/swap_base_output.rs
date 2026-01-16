use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SwapBaseOutput {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SwapBaseOutputInstructionAccounts {
    pub payer: Address,
    pub authority: Address,
    pub amm_config: Address,
    pub pool_state: Address,
    pub input_token_account: Address,
    pub output_token_account: Address,
    pub input_vault: Address,
    pub output_vault: Address,
    pub input_token_program: Address,
    pub output_token_program: Address,
    pub input_token_mint: Address,
    pub output_token_mint: Address,
    pub observation_state: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SwapBaseOutput {
    pub const DISCRIMINATOR: [u8; 8] = [55, 217, 98, 86, 163, 74, 180, 173];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SwapBaseOutput {
    type ArrangedAccounts = SwapBaseOutputInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let input_token_account = next_account(&mut iter)?;
        let output_token_account = next_account(&mut iter)?;
        let input_vault = next_account(&mut iter)?;
        let output_vault = next_account(&mut iter)?;
        let input_token_program = next_account(&mut iter)?;
        let output_token_program = next_account(&mut iter)?;
        let input_token_mint = next_account(&mut iter)?;
        let output_token_mint = next_account(&mut iter)?;
        let observation_state = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SwapBaseOutputInstructionAccounts {
            payer,
            authority,
            amm_config,
            pool_state,
            input_token_account,
            output_token_account,
            input_vault,
            output_vault,
            input_token_program,
            output_token_program,
            input_token_mint,
            output_token_mint,
            observation_state,
            remaining: remaining.to_vec(),
        })
    }
}
