use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::PlatformParams;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePlatformConfig {
    pub platform_params: PlatformParams,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePlatformConfigInstructionAccounts {
    pub platform_admin: Address,
    pub platform_fee_wallet: Address,
    pub platform_nft_wallet: Address,
    pub platform_config: Address,
    pub cpswap_config: Address,
    pub system_program: Address,
    pub transfer_fee_extension_authority: Address,
    pub platform_vesting_wallet: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePlatformConfig {
    pub const DISCRIMINATOR: [u8; 8] = [176, 90, 196, 175, 253, 113, 220, 20];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePlatformConfig {
    type ArrangedAccounts = CreatePlatformConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let platform_admin = next_account(&mut iter)?;
        let platform_fee_wallet = next_account(&mut iter)?;
        let platform_nft_wallet = next_account(&mut iter)?;
        let platform_config = next_account(&mut iter)?;
        let cpswap_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let transfer_fee_extension_authority = next_account(&mut iter)?;
        let platform_vesting_wallet = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePlatformConfigInstructionAccounts {
            platform_admin,
            platform_fee_wallet,
            platform_nft_wallet,
            platform_config,
            cpswap_config,
            system_program,
            transfer_fee_extension_authority,
            platform_vesting_wallet,
            remaining: remaining.to_vec(),
        })
    }
}
