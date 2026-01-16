use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CollectProtocolFee {
    pub amount_0_requested: u64,
    pub amount_1_requested: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CollectProtocolFeeInstructionAccounts {
    pub owner: Address,
    pub authority: Address,
    pub pool_state: Address,
    pub amm_config: Address,
    pub token_0_vault: Address,
    pub token_1_vault: Address,
    pub vault_0_mint: Address,
    pub vault_1_mint: Address,
    pub recipient_token_0_account: Address,
    pub recipient_token_1_account: Address,
    pub token_program: Address,
    pub token_program_2022: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CollectProtocolFee {
    pub const DISCRIMINATOR: [u8; 8] = [136, 136, 252, 221, 194, 66, 126, 89];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CollectProtocolFee {
    type ArrangedAccounts = CollectProtocolFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let owner = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let pool_state = next_account(&mut iter)?;
        let amm_config = next_account(&mut iter)?;
        let token_0_vault = next_account(&mut iter)?;
        let token_1_vault = next_account(&mut iter)?;
        let vault_0_mint = next_account(&mut iter)?;
        let vault_1_mint = next_account(&mut iter)?;
        let recipient_token_0_account = next_account(&mut iter)?;
        let recipient_token_1_account = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let token_program_2022 = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CollectProtocolFeeInstructionAccounts {
            owner,
            authority,
            pool_state,
            amm_config,
            token_0_vault,
            token_1_vault,
            vault_0_mint,
            vault_1_mint,
            recipient_token_0_account,
            recipient_token_1_account,
            token_program,
            token_program_2022,
            remaining: remaining.to_vec(),
        })
    }
}
