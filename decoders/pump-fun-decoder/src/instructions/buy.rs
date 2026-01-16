use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Buy {
    pub amount: u64,
    pub max_sol_cost: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuyInstructionAccounts {
    pub global: Address,
    pub fee_recipient: Address,
    pub mint: Address,
    pub bonding_curve: Address,
    pub associated_bonding_curve: Address,
    pub associated_user: Address,
    pub user: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub creator_vault: Address,
    pub event_authority: Address,
    pub program: Address,
    pub global_volume_accumulator: Address,
    pub user_volume_accumulator: Address,
    pub fee_config: Address,
    pub fee_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Buy {
    pub const DISCRIMINATOR: [u8; 8] = [102, 6, 61, 18, 1, 218, 235, 234];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Buy {
    type ArrangedAccounts = BuyInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let global = next_account(&mut iter)?;
        let fee_recipient = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let associated_user = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let creator_vault = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let global_volume_accumulator = next_account(&mut iter)?;
        let user_volume_accumulator = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let fee_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(BuyInstructionAccounts {
            global,
            fee_recipient,
            mint,
            bonding_curve,
            associated_bonding_curve,
            associated_user,
            user,
            system_program,
            token_program,
            creator_vault,
            event_authority,
            program,
            global_volume_accumulator,
            user_volume_accumulator,
            fee_config,
            fee_program,
            remaining: remaining.to_vec(),
        })
    }
}
