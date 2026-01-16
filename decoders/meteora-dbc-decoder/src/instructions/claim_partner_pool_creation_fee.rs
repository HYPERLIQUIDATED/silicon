use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimPartnerPoolCreationFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimPartnerPoolCreationFeeInstructionAccounts {
    pub config: Address,
    pub pool: Address,
    pub fee_claimer: Address,
    pub fee_receiver: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimPartnerPoolCreationFee {
    pub const DISCRIMINATOR: [u8; 8] = [250, 238, 26, 4, 139, 10, 101, 248];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimPartnerPoolCreationFee {
    type ArrangedAccounts = ClaimPartnerPoolCreationFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let fee_claimer = next_account(&mut iter)?;
        let fee_receiver = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimPartnerPoolCreationFeeInstructionAccounts {
            config,
            pool,
            fee_claimer,
            fee_receiver,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
