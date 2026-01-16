use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    AdminSetCreatorEvent, AdminSetIdlAuthorityEvent, AdminUpdateTokenIncentivesEvent,
    ClaimTokenIncentivesEvent, CloseUserVolumeAccumulatorEvent, CollectCreatorFeeEvent,
    CompleteEvent, CompletePumpAmmMigrationEvent, CreateEvent, DistributeCreatorFeesEvent,
    ExtendAccountEvent, InitUserVolumeAccumulatorEvent, MigrateBondingCurveCreatorEvent,
    MinimumDistributableFeeEvent, ReservedFeeRecipientsEvent, SetCreatorEvent,
    SetMetaplexCreatorEvent, SetParamsEvent, SyncUserVolumeAccumulatorEvent, TradeEvent,
    UpdateGlobalAuthorityEvent, UpdateMayhemVirtualParamsEvent,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    AdminSetCreatorEvent(AdminSetCreatorEvent),
    AdminSetIdlAuthorityEvent(AdminSetIdlAuthorityEvent),
    AdminUpdateTokenIncentivesEvent(AdminUpdateTokenIncentivesEvent),
    ClaimTokenIncentivesEvent(ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulatorEvent(CloseUserVolumeAccumulatorEvent),
    CollectCreatorFeeEvent(CollectCreatorFeeEvent),
    CompleteEvent(CompleteEvent),
    CompletePumpAmmMigrationEvent(CompletePumpAmmMigrationEvent),
    CreateEvent(CreateEvent),
    DistributeCreatorFeesEvent(DistributeCreatorFeesEvent),
    ExtendAccountEvent(ExtendAccountEvent),
    InitUserVolumeAccumulatorEvent(InitUserVolumeAccumulatorEvent),
    MigrateBondingCurveCreatorEvent(MigrateBondingCurveCreatorEvent),
    MinimumDistributableFeeEvent(MinimumDistributableFeeEvent),
    ReservedFeeRecipientsEvent(ReservedFeeRecipientsEvent),
    SetCreatorEvent(SetCreatorEvent),
    SetMetaplexCreatorEvent(SetMetaplexCreatorEvent),
    SetParamsEvent(SetParamsEvent),
    SyncUserVolumeAccumulatorEvent(SyncUserVolumeAccumulatorEvent),
    TradeEvent(TradeEvent),
    UpdateGlobalAuthorityEvent(UpdateGlobalAuthorityEvent),
    UpdateMayhemVirtualParamsEvent(UpdateMayhemVirtualParamsEvent),
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
            if let Some(decoded) = AdminSetCreatorEvent::decode(data) {
                return Some(CpiEvent::AdminSetCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = AdminSetIdlAuthorityEvent::decode(data) {
                return Some(CpiEvent::AdminSetIdlAuthorityEvent(decoded));
            }
        }

        {
            if let Some(decoded) = AdminUpdateTokenIncentivesEvent::decode(data) {
                return Some(CpiEvent::AdminUpdateTokenIncentivesEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ClaimTokenIncentivesEvent::decode(data) {
                return Some(CpiEvent::ClaimTokenIncentivesEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CloseUserVolumeAccumulatorEvent::decode(data) {
                return Some(CpiEvent::CloseUserVolumeAccumulatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CollectCreatorFeeEvent::decode(data) {
                return Some(CpiEvent::CollectCreatorFeeEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CompleteEvent::decode(data) {
                return Some(CpiEvent::CompleteEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CompletePumpAmmMigrationEvent::decode(data) {
                return Some(CpiEvent::CompletePumpAmmMigrationEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CreateEvent::decode(data) {
                return Some(CpiEvent::CreateEvent(decoded));
            }
        }

        {
            if let Some(decoded) = DistributeCreatorFeesEvent::decode(data) {
                return Some(CpiEvent::DistributeCreatorFeesEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ExtendAccountEvent::decode(data) {
                return Some(CpiEvent::ExtendAccountEvent(decoded));
            }
        }

        {
            if let Some(decoded) = InitUserVolumeAccumulatorEvent::decode(data) {
                return Some(CpiEvent::InitUserVolumeAccumulatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = MigrateBondingCurveCreatorEvent::decode(data) {
                return Some(CpiEvent::MigrateBondingCurveCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = MinimumDistributableFeeEvent::decode(data) {
                return Some(CpiEvent::MinimumDistributableFeeEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ReservedFeeRecipientsEvent::decode(data) {
                return Some(CpiEvent::ReservedFeeRecipientsEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SetCreatorEvent::decode(data) {
                return Some(CpiEvent::SetCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SetMetaplexCreatorEvent::decode(data) {
                return Some(CpiEvent::SetMetaplexCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SetParamsEvent::decode(data) {
                return Some(CpiEvent::SetParamsEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SyncUserVolumeAccumulatorEvent::decode(data) {
                return Some(CpiEvent::SyncUserVolumeAccumulatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = TradeEvent::decode(data) {
                return Some(CpiEvent::TradeEvent(decoded));
            }
        }

        {
            if let Some(decoded) = UpdateGlobalAuthorityEvent::decode(data) {
                return Some(CpiEvent::UpdateGlobalAuthorityEvent(decoded));
            }
        }

        {
            if let Some(decoded) = UpdateMayhemVirtualParamsEvent::decode(data) {
                return Some(CpiEvent::UpdateMayhemVirtualParamsEvent(decoded));
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
