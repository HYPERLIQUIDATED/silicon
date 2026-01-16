use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ZapProtocolFee {
    pub max_amount: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZapProtocolFeeInstructionAccounts {
    pub pool_authority: Address,
    pub pool: Address,
    pub token_vault: Address,
    pub token_mint: Address,
    pub receiver_token: Address,
    pub operator: Address,
    pub signer: Address,
    pub token_program: Address,
    pub sysvar_instructions: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ZapProtocolFee {
    pub const DISCRIMINATOR: [u8; 8] = [213, 155, 187, 34, 56, 182, 91, 240];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ZapProtocolFee {
    type ArrangedAccounts = ZapProtocolFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let token_vault = next_account(&mut iter)?;
        let token_mint = next_account(&mut iter)?;
        let receiver_token = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let sysvar_instructions = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ZapProtocolFeeInstructionAccounts {
            pool_authority,
            pool,
            token_vault,
            token_mint,
            receiver_token,
            operator,
            signer,
            token_program,
            sysvar_instructions,
            remaining: remaining.to_vec(),
        })
    }
}
