use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    BagsFeeSharePartnerClaimEvent, BagsFeeShareProtocolClaimEvent, BagsFeeShareUserClaimEvent,
    FeeConfigSnapshotEvent, FeeConfigUpdatedEvent, PartnerAccumulatedEvent,
    PartnerConfigCreatedEvent, PartnerConfigUpdatedEvent, PartnerConfigV2SnapshotEvent,
    PlatformAccumulatedEvent, PlatformClaimedEvent, ProgramConfigAdminUpdatedEvent,
    ProgramConfigInitializedEvent, ProgramConfigUpdatedEvent,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    BagsFeeSharePartnerClaimEvent(BagsFeeSharePartnerClaimEvent),
    BagsFeeShareProtocolClaimEvent(BagsFeeShareProtocolClaimEvent),
    BagsFeeShareUserClaimEvent(BagsFeeShareUserClaimEvent),
    FeeConfigSnapshotEvent(FeeConfigSnapshotEvent),
    FeeConfigUpdatedEvent(FeeConfigUpdatedEvent),
    PartnerAccumulatedEvent(PartnerAccumulatedEvent),
    PartnerConfigCreatedEvent(PartnerConfigCreatedEvent),
    PartnerConfigUpdatedEvent(PartnerConfigUpdatedEvent),
    PartnerConfigV2SnapshotEvent(PartnerConfigV2SnapshotEvent),
    PlatformAccumulatedEvent(PlatformAccumulatedEvent),
    PlatformClaimedEvent(PlatformClaimedEvent),
    ProgramConfigAdminUpdatedEvent(ProgramConfigAdminUpdatedEvent),
    ProgramConfigInitializedEvent(ProgramConfigInitializedEvent),
    ProgramConfigUpdatedEvent(ProgramConfigUpdatedEvent),
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
            if let Some(decoded) = BagsFeeSharePartnerClaimEvent::decode(data) {
                return Some(CpiEvent::BagsFeeSharePartnerClaimEvent(decoded));
            }
        }

        {
            if let Some(decoded) = BagsFeeShareProtocolClaimEvent::decode(data) {
                return Some(CpiEvent::BagsFeeShareProtocolClaimEvent(decoded));
            }
        }

        {
            if let Some(decoded) = BagsFeeShareUserClaimEvent::decode(data) {
                return Some(CpiEvent::BagsFeeShareUserClaimEvent(decoded));
            }
        }

        {
            if let Some(decoded) = FeeConfigSnapshotEvent::decode(data) {
                return Some(CpiEvent::FeeConfigSnapshotEvent(decoded));
            }
        }

        {
            if let Some(decoded) = FeeConfigUpdatedEvent::decode(data) {
                return Some(CpiEvent::FeeConfigUpdatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PartnerAccumulatedEvent::decode(data) {
                return Some(CpiEvent::PartnerAccumulatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PartnerConfigCreatedEvent::decode(data) {
                return Some(CpiEvent::PartnerConfigCreatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PartnerConfigUpdatedEvent::decode(data) {
                return Some(CpiEvent::PartnerConfigUpdatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PartnerConfigV2SnapshotEvent::decode(data) {
                return Some(CpiEvent::PartnerConfigV2SnapshotEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PlatformAccumulatedEvent::decode(data) {
                return Some(CpiEvent::PlatformAccumulatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = PlatformClaimedEvent::decode(data) {
                return Some(CpiEvent::PlatformClaimedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ProgramConfigAdminUpdatedEvent::decode(data) {
                return Some(CpiEvent::ProgramConfigAdminUpdatedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ProgramConfigInitializedEvent::decode(data) {
                return Some(CpiEvent::ProgramConfigInitializedEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ProgramConfigUpdatedEvent::decode(data) {
                return Some(CpiEvent::ProgramConfigUpdatedEvent(decoded));
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
