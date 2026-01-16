use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct TransferCreatorFeesToPump {}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferCreatorFeesToPumpInstructionAccounts {
    pub wsol_mint: Address,
    pub token_program: Address,
    pub system_program: Address,
    pub associated_token_program: Address,
    pub coin_creator: Address,
    pub coin_creator_vault_authority: Address,
    pub coin_creator_vault_ata: Address,
    pub pump_creator_vault: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl TransferCreatorFeesToPump {
    pub const DISCRIMINATOR: [u8; 8] = [139, 52, 134, 85, 228, 229, 108, 241];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for TransferCreatorFeesToPump {
    type ArrangedAccounts = TransferCreatorFeesToPumpInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let wsol_mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let coin_creator = next_account(&mut iter)?;
        let coin_creator_vault_authority = next_account(&mut iter)?;
        let coin_creator_vault_ata = next_account(&mut iter)?;
        let pump_creator_vault = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(TransferCreatorFeesToPumpInstructionAccounts {
            wsol_mint,
            token_program,
            system_program,
            associated_token_program,
            coin_creator,
            coin_creator_vault_authority,
            coin_creator_vault_ata,
            pump_creator_vault,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
