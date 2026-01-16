use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ClaimProtocolPoolCreationFee {}

#[derive(Debug, Clone, PartialEq)]
pub struct ClaimProtocolPoolCreationFeeInstructionAccounts {
    pub config: Address,
    pub pool: Address,
    pub claim_fee_operator: Address,
    pub signer: Address,
    pub treasury: Address,
    pub event_authority: Address,
    pub program: Address,
    pub remaining: Vec<AccountMeta>,
}

impl ClaimProtocolPoolCreationFee {
    pub const DISCRIMINATOR: [u8; 8] = [114, 205, 83, 188, 240, 153, 25, 54];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for ClaimProtocolPoolCreationFee {
    type ArrangedAccounts = ClaimProtocolPoolCreationFeeInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let config = next_account(&mut iter)?;
        let pool = next_account(&mut iter)?;
        let claim_fee_operator = next_account(&mut iter)?;
        let signer = next_account(&mut iter)?;
        let treasury = next_account(&mut iter)?;
        let event_authority = next_account(&mut iter)?;
        let program = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(ClaimProtocolPoolCreationFeeInstructionAccounts {
            config,
            pool,
            claim_fee_operator,
            signer,
            treasury,
            event_authority,
            program,
            remaining: remaining.to_vec(),
        })
    }
}
