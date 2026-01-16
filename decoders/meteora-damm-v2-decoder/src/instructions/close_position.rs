use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClosePosition {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosePositionInstructionAccounts {
    pub position_nft_mint: Address,
    pub position_nft_account: Address,
    pub pool: Address,
    pub position: Address,
    pub pool_authority: Address,
    pub rent_receiver: Address,
    pub owner: Address,
    pub token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClosePosition {
    pub const DISCRIMINATOR: [u8; 8] = [123, 134, 81, 0, 49, 68, 98, 98];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClosePosition {
    type ArrangedAccounts = ClosePositionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let position_nft_mint = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let owner = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClosePositionInstructionAccounts {
            position_nft_mint,
            position_nft_account,
            pool,
            position,
            pool_authority,
            rent_receiver,
            owner,
            token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
