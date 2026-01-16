use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateAmmConfig {
    pub param: u8,
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAmmConfigInstructionAccounts {
    pub owner: Address,
    pub amm_config: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateAmmConfig {
    pub const DISCRIMINATOR: [u8; 8] = [49, 60, 174, 136, 154, 28, 116, 200];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateAmmConfig {
    type ArrangedAccounts = UpdateAmmConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateAmmConfigInstructionAccounts {
            owner,
            amm_config,
            remaining: remaining.to_vec(),
        })
    }
}
