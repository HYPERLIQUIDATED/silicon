use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetCoinCreator {
    pub coin_creator: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AdminSetCoinCreatorInstructionAccounts {
    pub admin_set_coin_creator_authority: Address,
    pub global_config: Address,
    pub pool: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl AdminSetCoinCreator {
    pub const DISCRIMINATOR: [u8; 8] = [242, 40, 117, 145, 73, 96, 105, 104];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for AdminSetCoinCreator {
    type ArrangedAccounts = AdminSetCoinCreatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let admin_set_coin_creator_authority = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(AdminSetCoinCreatorInstructionAccounts {
            admin_set_coin_creator_authority,
            global_config,
            pool,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
