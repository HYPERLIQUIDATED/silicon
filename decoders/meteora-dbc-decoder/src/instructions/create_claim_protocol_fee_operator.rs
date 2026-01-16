use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateClaimProtocolFeeOperator {}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateClaimProtocolFeeOperatorInstructionAccounts {
    pub claim_fee_operator: Address,
    pub operator: Address,
    pub signer: Address,
    pub payer: Address,
    pub system_program: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CreateClaimProtocolFeeOperator {
    pub const DISCRIMINATOR: [u8; 8] = [51, 19, 150, 252, 105, 157, 48, 91];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CreateClaimProtocolFeeOperator {
    type ArrangedAccounts = CreateClaimProtocolFeeOperatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let claim_fee_operator = next_account(&mut iter)?;
        let operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let payer = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CreateClaimProtocolFeeOperatorInstructionAccounts {
            claim_fee_operator,
            operator,
            signer,
            payer,
            system_program,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
