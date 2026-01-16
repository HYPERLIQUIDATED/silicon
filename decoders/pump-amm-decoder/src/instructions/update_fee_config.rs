use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct UpdateFeeConfig {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Address; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateFeeConfigInstructionAccounts {
    pub admin: Address,
    pub global_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl UpdateFeeConfig {
    pub const DISCRIMINATOR: [u8; 8] = [104, 184, 103, 242, 88, 151, 107, 20];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for UpdateFeeConfig {
    type ArrangedAccounts = UpdateFeeConfigInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(UpdateFeeConfigInstructionAccounts {
            admin,
            global_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
