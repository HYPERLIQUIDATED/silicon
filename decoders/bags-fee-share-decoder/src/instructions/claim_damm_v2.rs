use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimDammV2 {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimDammV2InstructionAccounts {
    pub payer: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub fee_share_authority_quote_ata: Address,
    pub fee_share_authority_base_ata: Address,
    pub program_config: Address,
    pub platform_vault: Address,
    pub partner_config: Option<Address>,
    pub partner: Option<Address>,
    pub partner_config_quote_ata: Option<Address>,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub damm_program: Address,
    pub pool_authority: Address,
    pub pool: Address,
    pub position: Address,
    pub base_vault: Address,
    pub quote_vault: Address,
    pub position_nft_account: Address,
    pub damm_event_authority: Address,
    pub token_base_program: Address,
    pub token_quote_program: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimDammV2 {
    pub const DISCRIMINATOR: [u8; 8] = [232, 175, 106, 19, 168, 54, 186, 108];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimDammV2 {
    type ArrangedAccounts = ClaimDammV2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let payer = next_account(&mut iter)?;
        let fee_share_config = next_account(&mut iter)?;
        let fee_share_authority = next_account(&mut iter)?;
        let fee_share_authority_quote_ata = next_account(&mut iter)?;
        let fee_share_authority_base_ata = next_account(&mut iter)?;
        let program_config = next_account(&mut iter)?;
        let platform_vault = next_account(&mut iter)?;
        let partner_config = next_account(&mut iter);
        let partner = next_account(&mut iter);
        let partner_config_quote_ata = next_account(&mut iter);
        let base_mint = next_account(&mut iter)?;
        let quote_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let damm_program = next_account(&mut iter)?;
        let pool_authority = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let position = next_account(&mut iter)?;
        let base_vault = next_account(&mut iter)?;
        let quote_vault = next_account(&mut iter)?;
        let position_nft_account = next_account(&mut iter)?;
        let damm_event_authority = next_account(&mut iter)?;
        let token_base_program = next_account(&mut iter)?;
        let token_quote_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimDammV2InstructionAccounts {
            payer,
            fee_share_config,
            fee_share_authority,
            fee_share_authority_quote_ata,
            fee_share_authority_base_ata,
            program_config,
            platform_vault,
            partner_config,
            partner,
            partner_config_quote_ata,
            base_mint,
            quote_mint,
            token_program,
            associated_token_program,
            damm_program,
            pool_authority,
            pool,
            position,
            base_vault,
            quote_vault,
            position_nft_account,
            damm_event_authority,
            token_base_program,
            token_quote_program,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
