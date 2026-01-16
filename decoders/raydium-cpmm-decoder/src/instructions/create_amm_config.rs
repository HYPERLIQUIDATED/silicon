use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateAmmConfig {
    pub index: u16,
    pub trade_fee_rate: u64,
    pub protocol_fee_rate: u64,
    pub fund_fee_rate: u64,
    pub create_pool_fee: u64,
    pub creator_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateAmmConfigInstructionAccounts {
    pub owner: Address,
    pub amm_config: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateAmmConfig {
    pub const DISCRIMINATOR: [u8; 8] = [137, 52, 237, 212, 215, 117, 108, 104];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateAmmConfig {
    type ArrangedAccounts = CreateAmmConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateAmmConfigInstructionAccounts {
            owner,
            amm_config,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
