use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateConfig {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Address; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateConfigInstructionAccounts {
    pub admin: Address,
    pub global_config: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
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

        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateConfigInstructionAccounts {
            admin,
            global_config,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
