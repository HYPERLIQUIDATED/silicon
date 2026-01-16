use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct WithdrawLeftover {}

#[derive(Debug, Clone, PartialEq)]
pub struct WithdrawLeftoverInstructionAccounts {
    pub pool_authority: Address,
    pub config: Address,
    pub virtual_pool: Address,
    pub token_base_account: Address,
    pub base_vault: Address,
    pub base_mint: Address,
    pub leftover_receiver: Address,
    pub token_base_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl WithdrawLeftover {
    pub const DISCRIMINATOR: [u8; 8] = [20, 198, 202, 237, 235, 243, 183, 66];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for WithdrawLeftover {
    type ArrangedAccounts = WithdrawLeftoverInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool_authority = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let virtual_pool = next_account(&mut iter)?;
        let token_base_account = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let leftover_receiver = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(WithdrawLeftoverInstructionAccounts {
            pool_authority,
            config,
            virtual_pool,
            token_base_account,
            base_vault,
            base_mint,
            leftover_receiver,
            token_base_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
