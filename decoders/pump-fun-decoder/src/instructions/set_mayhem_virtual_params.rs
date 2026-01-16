use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct SetMayhemVirtualParams {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetMayhemVirtualParamsInstructionAccounts {
    pub sol_vault_authority: Address,
    pub mayhem_token_vault: Address,
    pub mint: Address,
    pub global: Address,
    pub bonding_curve: Address,
    pub token_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl SetMayhemVirtualParams {
    pub const DISCRIMINATOR: [u8; 8] = [61, 169, 188, 191, 153, 149, 42, 97];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for SetMayhemVirtualParams {
    type ArrangedAccounts = SetMayhemVirtualParamsInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let sol_vault_authority = next_account(&mut iter)?;
        let mayhem_token_vault = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let global = next_account(&mut iter)?;
        let bonding_curve = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(SetMayhemVirtualParamsInstructionAccounts {
            sol_vault_authority,
            mayhem_token_vault,
            mint,
            global,
            bonding_curve,
            token_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
