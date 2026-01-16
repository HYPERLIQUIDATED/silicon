use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Create {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Address,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateInstructionAccounts {
    pub mint: Address,
    pub mint_authority: Address,
    pub bonding_curve: Address,
    pub associated_bonding_curve: Address,
    pub global: Address,
    pub mpl_token_metadata: Address,
    pub metadata: Address,
    pub user: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub rent: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Create {
    pub const DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Create {
    type ArrangedAccounts = CreateInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let mpl_token_metadata = next_account(&mut iter)?;
        let metadata = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateInstructionAccounts {
            mint,
            mint_authority,
            bonding_curve,
            associated_bonding_curve,
            global,
            mpl_token_metadata,
            metadata,
            user,
            system_program,
            token_program,
            associated_token_program,
            rent,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
