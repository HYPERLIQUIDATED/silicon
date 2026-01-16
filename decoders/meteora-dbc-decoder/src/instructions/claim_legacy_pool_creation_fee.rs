use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimLegacyPoolCreationFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimLegacyPoolCreationFeeInstructionAccounts {
    pub pool: Address,
    pub claim_fee_operator: Address,
    pub signer: Address,
    pub treasury: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimLegacyPoolCreationFee {
    pub const DISCRIMINATOR: [u8; 8] = [96, 11, 187, 225, 54, 117, 161, 134];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimLegacyPoolCreationFee {
    type ArrangedAccounts = ClaimLegacyPoolCreationFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pool = next_account(&mut iter)?;
        let claim_fee_operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let treasury = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimLegacyPoolCreationFeeInstructionAccounts {
            pool,
            claim_fee_operator,
            signer,
            treasury,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
