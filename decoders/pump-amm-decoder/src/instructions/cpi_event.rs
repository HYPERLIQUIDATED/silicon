use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    AdminSetCoinCreatorEvent, AdminUpdateTokenIncentivesEvent, BuyEvent, ClaimTokenIncentivesEvent,
    CloseUserVolumeAccumulatorEvent, CollectCoinCreatorFeeEvent, CreateConfigEvent,
    CreatePoolEvent, DepositEvent, DisableEvent, ExtendAccountEvent,
    InitUserVolumeAccumulatorEvent, MigratePoolCoinCreatorEvent, ReservedFeeRecipientsEvent,
    SellEvent, SetBondingCurveCoinCreatorEvent, SetMetaplexCoinCreatorEvent,
    SyncUserVolumeAccumulatorEvent, UpdateAdminEvent, UpdateFeeConfigEvent, WithdrawEvent,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    AdminSetCoinCreatorEvent(AdminSetCoinCreatorEvent),
    AdminUpdateTokenIncentivesEvent(AdminUpdateTokenIncentivesEvent),
    BuyEvent(BuyEvent),
    ClaimTokenIncentivesEvent(ClaimTokenIncentivesEvent),
    CloseUserVolumeAccumulatorEvent(CloseUserVolumeAccumulatorEvent),
    CollectCoinCreatorFeeEvent(CollectCoinCreatorFeeEvent),
    CreateConfigEvent(CreateConfigEvent),
    CreatePoolEvent(CreatePoolEvent),
    DepositEvent(DepositEvent),
    DisableEvent(DisableEvent),
    ExtendAccountEvent(ExtendAccountEvent),
    InitUserVolumeAccumulatorEvent(InitUserVolumeAccumulatorEvent),
    MigratePoolCoinCreatorEvent(MigratePoolCoinCreatorEvent),
    ReservedFeeRecipientsEvent(ReservedFeeRecipientsEvent),
    SellEvent(SellEvent),
    SetBondingCurveCoinCreatorEvent(SetBondingCurveCoinCreatorEvent),
    SetMetaplexCoinCreatorEvent(SetMetaplexCoinCreatorEvent),
    SyncUserVolumeAccumulatorEvent(SyncUserVolumeAccumulatorEvent),
    UpdateAdminEvent(UpdateAdminEvent),
    UpdateFeeConfigEvent(UpdateFeeConfigEvent),
    WithdrawEvent(WithdrawEvent),
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
            if let Some(decoded) = AdminSetCoinCreatorEvent::decode(data) {
                return Some(CpiEvent::AdminSetCoinCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = AdminUpdateTokenIncentivesEvent::decode(data) {
                return Some(CpiEvent::AdminUpdateTokenIncentivesEvent(decoded));
            }
        }

        {
            if let Some(decoded) = BuyEvent::decode(data) {
                return Some(CpiEvent::BuyEvent(decoded));
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
            if let Some(decoded) = CollectCoinCreatorFeeEvent::decode(data) {
                return Some(CpiEvent::CollectCoinCreatorFeeEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CreateConfigEvent::decode(data) {
                return Some(CpiEvent::CreateConfigEvent(decoded));
            }
        }

        {
            if let Some(decoded) = CreatePoolEvent::decode(data) {
                return Some(CpiEvent::CreatePoolEvent(decoded));
            }
        }

        {
            if let Some(decoded) = DepositEvent::decode(data) {
                return Some(CpiEvent::DepositEvent(decoded));
            }
        }

        {
            if let Some(decoded) = DisableEvent::decode(data) {
                return Some(CpiEvent::DisableEvent(decoded));
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
            if let Some(decoded) = MigratePoolCoinCreatorEvent::decode(data) {
                return Some(CpiEvent::MigratePoolCoinCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = ReservedFeeRecipientsEvent::decode(data) {
                return Some(CpiEvent::ReservedFeeRecipientsEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SellEvent::decode(data) {
                return Some(CpiEvent::SellEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SetBondingCurveCoinCreatorEvent::decode(data) {
                return Some(CpiEvent::SetBondingCurveCoinCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SetMetaplexCoinCreatorEvent::decode(data) {
                return Some(CpiEvent::SetMetaplexCoinCreatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = SyncUserVolumeAccumulatorEvent::decode(data) {
                return Some(CpiEvent::SyncUserVolumeAccumulatorEvent(decoded));
            }
        }

        {
            if let Some(decoded) = UpdateAdminEvent::decode(data) {
                return Some(CpiEvent::UpdateAdminEvent(decoded));
            }
        }

        {
            if let Some(decoded) = UpdateFeeConfigEvent::decode(data) {
                return Some(CpiEvent::UpdateFeeConfigEvent(decoded));
            }
        }

        {
            if let Some(decoded) = WithdrawEvent::decode(data) {
                return Some(CpiEvent::WithdrawEvent(decoded));
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
