use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimCreatorFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimCreatorFeeInstructionAccounts {
    pub creator: Address,
    pub fee_vault_authority: Address,
    pub creator_fee_vault: Address,
    pub recipient_token_account: Address,
    pub quote_mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimCreatorFee {
    pub const DISCRIMINATOR: [u8; 8] = [26, 97, 138, 203, 132, 171, 141, 252];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimCreatorFee {
    type ArrangedAccounts = ClaimCreatorFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let creator = next_account(&mut iter)?;
        let fee_vault_authority = next_account(&mut iter)?;
        let creator_fee_vault = next_account(&mut iter)?;
        let recipient_token_account = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimCreatorFeeInstructionAccounts {
            creator,
            fee_vault_authority,
            creator_fee_vault,
            recipient_token_account,
            quote_mint,
            token_program,
            system_program,
            associated_token_program,
            remaining: remaining.to_vec(),
        })
    }
}
