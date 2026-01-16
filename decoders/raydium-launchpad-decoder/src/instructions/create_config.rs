use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateConfig {
    pub curve_type: u8,
    pub index: u16,
    pub migrate_fee: u64,
    pub trade_fee_rate: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateConfigInstructionAccounts {
    pub owner: Address,
    pub global_config: Address,
    pub quote_token_mint: Address,
    pub protocol_fee_owner: Address,
    pub migrate_fee_owner: Address,
    pub migrate_to_amm_wallet: Address,
    pub migrate_to_cpswap_wallet: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateConfig {
    pub const DISCRIMINATOR: [u8; 8] = [201, 207, 243, 114, 75, 111, 47, 189];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateConfig {
    type ArrangedAccounts = CreateConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let quote_token_mint = next_account(&mut iter)?;
        let protocol_fee_owner = next_account(&mut iter)?;
        let migrate_fee_owner = next_account(&mut iter)?;
        let migrate_to_amm_wallet = next_account(&mut iter)?;
        let migrate_to_cpswap_wallet = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateConfigInstructionAccounts {
            owner,
            global_config,
            quote_token_mint,
            protocol_fee_owner,
            migrate_fee_owner,
            migrate_to_amm_wallet,
            migrate_to_cpswap_wallet,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
