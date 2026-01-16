use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct Sell {
    pub base_amount_in: u64,
    pub min_quote_amount_out: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SellInstructionAccounts {
    pub pool: Address,
    pub user: Address,
    pub global_config: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub user_base_token_account: Address,
    pub user_quote_token_account: Address,
    pub pool_base_token_account: Address,
    pub pool_quote_token_account: Address,
    pub protocol_fee_recipient: Address,
    pub protocol_fee_recipient_token_account: Address,
    pub base_token_program: Address,
    pub quote_token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub coin_creator_vault_ata: Address,
    pub coin_creator_vault_authority: Address,
    pub fee_config: Address,
    pub fee_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl Sell {
    pub const DISCRIMINATOR: [u8; 8] = [51, 230, 133, 164, 1, 127, 131, 173];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for Sell {
    type ArrangedAccounts = SellInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let user_base_token_account = next_account(&mut iter)?;
        let user_quote_token_account = next_account(&mut iter)?;
        let pool_base_token_account = next_account(&mut iter)?;
        let pool_quote_token_account = next_account(&mut iter)?;
        let protocol_fee_recipient = next_account(&mut iter)?;
        let protocol_fee_recipient_token_account = next_account(&mut iter)?;
        let base_token_program = next_account(&mut iter)?;
        let quote_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;
        let coin_creator_vault_ata = next_account(&mut iter)?;
        let coin_creator_vault_authority = next_account(&mut iter)?;
        let fee_config = next_account(&mut iter)?;
        let fee_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SellInstructionAccounts {
            pool,
            user,
            global_config,
            base_mint,
            quote_mint,
            user_base_token_account,
            user_quote_token_account,
            pool_base_token_account,
            pool_quote_token_account,
            protocol_fee_recipient,
            protocol_fee_recipient_token_account,
            base_token_program,
            quote_token_program,
            system_program,
            associated_token_program,
            event_authority,
            program,
            coin_creator_vault_ata,
            coin_creator_vault_authority,
            fee_config,
            fee_program,
            remaining: remaining.to_vec(),
        })
    }
}
