use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DistributeCreatorFees {}

#[derive(Debug, Clone, PartialEq)]
pub struct DistributeCreatorFeesInstructionAccounts {
    pub mint: Address,
    pub bonding_curve: Address,
    pub sharing_config: Address,
    pub creator_vault: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl DistributeCreatorFees {
    pub const DISCRIMINATOR: [u8; 8] = [165, 114, 103, 0, 121, 206, 247, 81];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for DistributeCreatorFees {
    type ArrangedAccounts = DistributeCreatorFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let sharing_config = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(DistributeCreatorFeesInstructionAccounts {
            mint,
            bonding_curve,
            sharing_config,
            creator_vault,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
