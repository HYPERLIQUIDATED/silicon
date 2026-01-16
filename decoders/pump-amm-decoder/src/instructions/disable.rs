use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Disable {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisableInstructionAccounts {
    pub admin: Address,
    pub global_config: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Disable {
    pub const DISCRIMINATOR: [u8; 8] = [185, 173, 187, 90, 216, 15, 238, 233];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Disable {
    type ArrangedAccounts = DisableInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(DisableInstructionAccounts {
            admin,
            global_config,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
