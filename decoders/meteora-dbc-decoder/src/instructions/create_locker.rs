use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateLocker {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateLockerInstructionAccounts {
    pub virtual_pool: Address,
    pub config: Address,
    pub pool_authority: Address,
    pub base_vault: Address,
    pub base_mint: Address,
    pub base: Address,
    pub creator: Address,
    pub escrow: Address,
    pub escrow_token: Address,
    pub payer: Address,
    pub token_program: Address,
    pub locker_program: Address,
    pub locker_event_authority: Address,
    pub system_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateLocker {
    pub const DISCRIMINATOR: [u8; 8] = [167, 90, 137, 154, 75, 47, 17, 84];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateLocker {
    type ArrangedAccounts = CreateLockerInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let virtual_pool = next_account(&mut iter)?;
        let config = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let base = next_account(&mut iter)?;
        let creator = next_account(&mut iter)?;
        let escrow = next_account(&mut iter)?;
        let escrow_token = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let locker_program = next_account(&mut iter)?;
        let locker_event_authority = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateLockerInstructionAccounts {
            virtual_pool,
            config,
            pool_authority,
            base_vault,
            base_mint,
            base,
            creator,
            escrow,
            escrow_token,
            payer,
            token_program,
            locker_program,
            locker_event_authority,
            system_program,
            remaining: remaining.to_vec(),
        })
    }
}
