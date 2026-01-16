use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigrateToAmm {
    pub base_lot_size: u64,
    pub quote_lot_size: u64,
    pub market_vault_signer_nonce: u8,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MigrateToAmmInstructionAccounts {
    pub payer: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub openbook_program: Address,
    pub market: Address,
    pub request_queue: Address,
    pub event_queue: Address,
    pub bids: Address,
    pub asks: Address,
    pub market_vault_signer: Address,
    pub market_base_vault: Address,
    pub market_quote_vault: Address,
    pub amm_program: Address,
    pub amm_pool: Address,
    pub amm_authority: Address,
    pub amm_open_orders: Address,
    pub amm_lp_mint: Address,
    pub amm_base_vault: Address,
    pub amm_quote_vault: Address,
    pub amm_target_orders: Address,
    pub amm_config: Address,
    pub amm_create_fee_destination: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub global_config: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub pool_lp_token: Address,
    pub spl_token_program: Address,
    pub associated_token_program: Address,
    pub system_program: Address,
    pub rent_program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl MigrateToAmm {
    pub const DISCRIMINATOR: [u8; 8] = [207, 82, 192, 145, 254, 207, 145, 223];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for MigrateToAmm {
    type ArrangedAccounts = MigrateToAmmInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let openbook_program = next_account(&mut iter)?;
        let market = next_account(&mut iter)?;
        let request_queue = next_account(&mut iter)?;
        let event_queue = next_account(&mut iter)?;
        let bids = next_account(&mut iter)?;
        let asks = next_account(&mut iter)?;
        let market_vault_signer = next_account(&mut iter)?;
        let market_base_vault = next_account(&mut iter)?;
        let market_quote_vault = next_account(&mut iter)?;
        let amm_program = next_account(&mut iter)?;
        let amm_pool = next_account(&mut iter)?;
        let amm_authority = next_account(&mut iter)?;
        let amm_open_orders = next_account(&mut iter)?;
        let amm_lp_mint = next_account(&mut iter)?;
        let amm_base_vault = next_account(&mut iter)?;
        let amm_quote_vault = next_account(&mut iter)?;
        let amm_target_orders = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let amm_create_fee_destination = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let global_config = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let pool_lp_token = next_account(&mut iter)?;
        let spl_token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let rent_program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(MigrateToAmmInstructionAccounts {
            payer,
            base_mint,
            quote_mint,
            openbook_program,
            market,
            request_queue,
            event_queue,
            bids,
            asks,
            market_vault_signer,
            market_base_vault,
            market_quote_vault,
            amm_program,
            amm_pool,
            amm_authority,
            amm_open_orders,
            amm_lp_mint,
            amm_base_vault,
            amm_quote_vault,
            amm_target_orders,
            amm_config,
            amm_create_fee_destination,
            authority,
            pool_state,
            global_config,
            base_vault,
            quote_vault,
            pool_lp_token,
            spl_token_program,
            associated_token_program,
            system_program,
            rent_program,
            remaining: remaining.to_vec(),
        })
    }
}
