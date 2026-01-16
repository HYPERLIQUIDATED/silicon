use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creator: Address,
    pub is_mayhem_mode: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateV2InstructionAccounts {
    pub mint: Address,
    pub mint_authority: Address,
    pub bonding_curve: Address,
    pub associated_bonding_curve: Address,
    pub global: Address,
    pub user: Address,
    pub system_program: Address,
    pub token_program: Address,
    pub associated_token_program: Address,
    pub mayhem_program_id: Address,
    pub global_params: Address,
    pub sol_vault: Address,
    pub mayhem_state: Address,
    pub mayhem_token_vault: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateV2 {
    pub const DISCRIMINATOR: [u8; 8] = [214, 144, 76, 236, 95, 139, 49, 180];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateV2 {
    type ArrangedAccounts = CreateV2InstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let mint = next_account(&mut iter)?;
        let mint_authority = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let associated_bonding_curve = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let user = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let mayhem_program_id = next_account(&mut iter)?;
        let global_params = next_account(&mut iter)?;
        let sol_vault = next_account(&mut iter)?;
        let mayhem_state = next_account(&mut iter)?;
        let mayhem_token_vault = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateV2InstructionAccounts {
            mint,
            mint_authority,
            bonding_curve,
            associated_bonding_curve,
            global,
            user,
            system_program,
            token_program,
            associated_token_program,
            mayhem_program_id,
            global_params,
            sol_vault,
            mayhem_state,
            mayhem_token_vault,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
