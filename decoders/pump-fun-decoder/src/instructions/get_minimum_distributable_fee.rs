use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct GetMinimumDistributableFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct GetMinimumDistributableFeeInstructionAccounts {
    pub mint: Address,
    pub bonding_curve: Address,
    pub sharing_config: Address,
    pub creator_vault: Address,
    pub remaining: Vec<AccountMeta>,
}

impl GetMinimumDistributableFee {
    pub const DISCRIMINATOR: [u8; 8] = [117, 225, 127, 202, 134, 95, 68, 35];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for GetMinimumDistributableFee {
    type ArrangedAccounts = GetMinimumDistributableFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let sharing_config = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(GetMinimumDistributableFeeInstructionAccounts {
            mint,
            bonding_curve,
            sharing_config,
            creator_vault,
            remaining: remaining.to_vec(),
        })
    }
}
