use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreatePosition {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatePositionInstructionAccounts {
    pub owner: Address,
    pub position_nft_mint: Address,
    pub position_nft_account: Address,
    pub pool: Address,
    pub position: Address,
    pub pool_authority: Address,
    pub payer: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreatePosition {
    pub const DISCRIMINATOR: [u8; 8] = [48, 215, 197, 153, 96, 203, 180, 133];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreatePosition {
    type ArrangedAccounts = CreatePositionInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let position_nft_mint = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreatePositionInstructionAccounts {
            owner,
            position_nft_mint,
            position_nft_account,
            pool,
            position,
            pool_authority,
            payer,
            token_program,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
