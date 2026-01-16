use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::types::DummyParams;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct DummyIx {
    pub ixs: DummyParams,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DummyIxInstructionAccounts {
    pub pod_aligned_fee_time_scheduler: Address,
    pub pod_aligned_fee_rate_limiter: Address,
    pub pod_aligned_fee_market_cap_scheduler: Address,
    pub remaining: Vec<AccountMeta>,
}

impl DummyIx {
    pub const DISCRIMINATOR: [u8; 8] = [234, 95, 176, 185, 7, 42, 35, 159];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}

impl ArrangeAccounts for DummyIx {
    type ArrangedAccounts = DummyIxInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let pod_aligned_fee_time_scheduler = next_account(&mut iter)?;
        let pod_aligned_fee_rate_limiter = next_account(&mut iter)?;
        let pod_aligned_fee_market_cap_scheduler = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(DummyIxInstructionAccounts {
            pod_aligned_fee_time_scheduler,
            pod_aligned_fee_rate_limiter,
            pod_aligned_fee_market_cap_scheduler,
            remaining: remaining.to_vec(),
        })
    }
}
