use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimPlatformFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimPlatformFeeInstructionAccounts {
    pub platform_fee_wallet: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub platform_config: Address,
    pub quote_vault: Address,
    pub recipient_token_account: Address,
    pub quote_mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimPlatformFee {
    pub const DISCRIMINATOR: [u8; 8] = [156, 39, 208, 135, 76, 237, 61, 72];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimPlatformFee {
    type ArrangedAccounts = ClaimPlatformFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_fee_wallet = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimPlatformFeeInstructionAccounts {
            platform_fee_wallet,
            authority,
            pool_state,
            platform_config,
            quote_vault,
            recipient_token_account,
            quote_mint,
            token_program,
            system_program,
            associated_token_program,
            remaining: remaining.to_vec(),
        })
    }
}
