use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateBondingCurveCreator {}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateBondingCurveCreatorInstructionAccounts {
    pub mint: Address,
    pub bonding_curve: Address,
    pub sharing_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateBondingCurveCreator {
    pub const DISCRIMINATOR: [u8; 8] = [87, 124, 52, 191, 52, 38, 214, 232];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateBondingCurveCreator {
    type ArrangedAccounts = MigrateBondingCurveCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let sharing_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateBondingCurveCreatorInstructionAccounts {
            mint,
            bonding_curve,
            sharing_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
