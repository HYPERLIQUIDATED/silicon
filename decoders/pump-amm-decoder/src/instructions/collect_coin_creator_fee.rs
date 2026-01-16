use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectCoinCreatorFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectCoinCreatorFeeInstructionAccounts {
    pub quote_mint: Address,
    pub quote_token_program: Address,
    pub coin_creator: Address,
    pub coin_creator_vault_authority: Address,
    pub coin_creator_vault_ata: Address,
    pub coin_creator_token_account: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CollectCoinCreatorFee {
    pub const DISCRIMINATOR: [u8; 8] = [160, 57, 89, 42, 181, 139, 43, 66];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CollectCoinCreatorFee {
    type ArrangedAccounts = CollectCoinCreatorFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let quote_mint = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let coin_creator = next_account(&mut iter)?;
        let coin_creator_vault_authority = next_account(&mut iter)?;
        let coin_creator_vault_ata = next_account(&mut iter)?;
        let coin_creator_token_account = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CollectCoinCreatorFeeInstructionAccounts {
            quote_mint,
            quote_token_program,
            coin_creator,
            coin_creator_vault_authority,
            coin_creator_vault_ata,
            coin_creator_token_account,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
