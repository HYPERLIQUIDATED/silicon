use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimPlatformFeeFromVault {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimPlatformFeeFromVaultInstructionAccounts {
    pub platform_fee_wallet: Address,
    pub fee_vault_authority: Address,
    pub platform_config: Address,
    pub platform_fee_vault: Address,
    pub recipient_token_account: Address,
    pub quote_mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimPlatformFeeFromVault {
    pub const DISCRIMINATOR: [u8; 8] = [117, 241, 198, 168, 248, 218, 80, 29];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimPlatformFeeFromVault {
    type ArrangedAccounts = ClaimPlatformFeeFromVaultInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_fee_wallet = next_account(&mut iter)?;
        let fee_vault_authority = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let platform_fee_vault = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimPlatformFeeFromVaultInstructionAccounts {
            platform_fee_wallet,
            fee_vault_authority,
            platform_config,
            platform_fee_vault,
            recipient_token_account,
            quote_mint,
            token_program,
            system_program,
            associated_token_program,
            remaining: remaining.to_vec(),
        })
    }
}
