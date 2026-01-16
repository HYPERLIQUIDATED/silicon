use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CloseClaimProtocolFeeOperator {}

#[derive(Debug, Clone, PartialEq)]
pub struct CloseClaimProtocolFeeOperatorInstructionAccounts {
    pub claim_fee_operator: Address,
    pub rent_receiver: Address,
    pub signer: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CloseClaimProtocolFeeOperator {
    pub const DISCRIMINATOR: [u8; 8] = [8, 41, 87, 35, 80, 48, 121, 26];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for CloseClaimProtocolFeeOperator {
    type ArrangedAccounts = CloseClaimProtocolFeeOperatorInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let claim_fee_operator = next_account(&mut iter)?;
        let rent_receiver = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CloseClaimProtocolFeeOperatorInstructionAccounts {
            claim_fee_operator,
            rent_receiver,
            signer,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
