use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    DAMMClaimedEvent, DBCClaimedEvent, FeeVaultsCreatedEvent, FeeVaultsTweakedEvent,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    DAMMClaimedEvent(DAMMClaimedEvent),
    DBCClaimedEvent(DBCClaimedEvent),
    FeeVaultsCreatedEvent(FeeVaultsCreatedEvent),
    FeeVaultsTweakedEvent(FeeVaultsTweakedEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CpiEventInstructionAccounts {
    pub event_authority: Address,
    pub remaining: Vec<AccountMeta>,
}

impl CpiEvent {
    pub const DISCRIMINATOR: [u8; 8] = [228, 69, 165, 46, 81, 203, 154, 29];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let data = data.strip_prefix(&Self::DISCRIMINATOR)?;

        {
            if let Some(decoded) = DAMMClaimedEvent::decode(data) {
                return Some(CpiEvent::DAMMClaimedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = DBCClaimedEvent::decode(data) {
                return Some(CpiEvent::DBCClaimedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = FeeVaultsCreatedEvent::decode(data) {
                return Some(CpiEvent::FeeVaultsCreatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = FeeVaultsTweakedEvent::decode(data) {
                return Some(CpiEvent::FeeVaultsTweakedEvent(decoded));
            }
        }

        None
    }
}

impl ArrangeAccounts for CpiEvent {
    type ArrangedAccounts = CpiEventInstructionAccounts;

    fn arrange_accounts(accounts: &[AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();

        let event_authority = next_account(&mut iter)?;

        let remaining = iter.as_slice();

        Some(CpiEventInstructionAccounts {
            event_authority,
            remaining: remaining.to_vec(),
        })
    }
}
