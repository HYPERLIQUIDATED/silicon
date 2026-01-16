use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    EvtClaimPartnerFee, EvtClaimPositionFee, EvtClaimProtocolFee, EvtClaimReward, EvtCloseConfig,
    EvtClosePosition, EvtCreateConfig, EvtCreateDynamicConfig, EvtCreatePosition,
    EvtCreateTokenBadge, EvtFundReward, EvtInitializePool, EvtInitializeReward, EvtLiquidityChange,
    EvtLockPosition, EvtPermanentLockPosition, EvtSetPoolStatus, EvtSplitPosition2,
    EvtSplitPosition3, EvtSwap2, EvtUpdatePoolFees, EvtUpdateRewardDuration, EvtUpdateRewardFunder,
    EvtWithdrawIneligibleReward,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    EvtClaimPartnerFee(EvtClaimPartnerFee),
    EvtClaimPositionFee(EvtClaimPositionFee),
    EvtClaimProtocolFee(EvtClaimProtocolFee),
    EvtClaimReward(EvtClaimReward),
    EvtCloseConfig(EvtCloseConfig),
    EvtClosePosition(EvtClosePosition),
    EvtCreateConfig(EvtCreateConfig),
    EvtCreateDynamicConfig(EvtCreateDynamicConfig),
    EvtCreatePosition(EvtCreatePosition),
    EvtCreateTokenBadge(EvtCreateTokenBadge),
    EvtFundReward(EvtFundReward),
    EvtInitializePool(EvtInitializePool),
    EvtInitializeReward(EvtInitializeReward),
    EvtLiquidityChange(EvtLiquidityChange),
    EvtLockPosition(EvtLockPosition),
    EvtPermanentLockPosition(EvtPermanentLockPosition),
    EvtSetPoolStatus(EvtSetPoolStatus),
    EvtSplitPosition2(EvtSplitPosition2),
    EvtSplitPosition3(EvtSplitPosition3),
    EvtSwap2(EvtSwap2),
    EvtUpdatePoolFees(EvtUpdatePoolFees),
    EvtUpdateRewardDuration(EvtUpdateRewardDuration),
    EvtUpdateRewardFunder(EvtUpdateRewardFunder),
    EvtWithdrawIneligibleReward(EvtWithdrawIneligibleReward),
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
            if let Some(decoded) = EvtClaimPartnerFee::decode(data) {
                return Some(CpiEvent::EvtClaimPartnerFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimPositionFee::decode(data) {
                return Some(CpiEvent::EvtClaimPositionFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimProtocolFee::decode(data) {
                return Some(CpiEvent::EvtClaimProtocolFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimReward::decode(data) {
                return Some(CpiEvent::EvtClaimReward(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCloseConfig::decode(data) {
                return Some(CpiEvent::EvtCloseConfig(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClosePosition::decode(data) {
                return Some(CpiEvent::EvtClosePosition(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateConfig::decode(data) {
                return Some(CpiEvent::EvtCreateConfig(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateDynamicConfig::decode(data) {
                return Some(CpiEvent::EvtCreateDynamicConfig(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreatePosition::decode(data) {
                return Some(CpiEvent::EvtCreatePosition(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateTokenBadge::decode(data) {
                return Some(CpiEvent::EvtCreateTokenBadge(decoded));
            }
        }

        {
            if let Some(decoded) = EvtFundReward::decode(data) {
                return Some(CpiEvent::EvtFundReward(decoded));
            }
        }

        {
            if let Some(decoded) = EvtInitializePool::decode(data) {
                return Some(CpiEvent::EvtInitializePool(decoded));
            }
        }

        {
            if let Some(decoded) = EvtInitializeReward::decode(data) {
                return Some(CpiEvent::EvtInitializeReward(decoded));
            }
        }

        {
            if let Some(decoded) = EvtLiquidityChange::decode(data) {
                return Some(CpiEvent::EvtLiquidityChange(decoded));
            }
        }

        {
            if let Some(decoded) = EvtLockPosition::decode(data) {
                return Some(CpiEvent::EvtLockPosition(decoded));
            }
        }

        {
            if let Some(decoded) = EvtPermanentLockPosition::decode(data) {
                return Some(CpiEvent::EvtPermanentLockPosition(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSetPoolStatus::decode(data) {
                return Some(CpiEvent::EvtSetPoolStatus(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSplitPosition2::decode(data) {
                return Some(CpiEvent::EvtSplitPosition2(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSplitPosition3::decode(data) {
                return Some(CpiEvent::EvtSplitPosition3(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSwap2::decode(data) {
                return Some(CpiEvent::EvtSwap2(decoded));
            }
        }

        {
            if let Some(decoded) = EvtUpdatePoolFees::decode(data) {
                return Some(CpiEvent::EvtUpdatePoolFees(decoded));
            }
        }

        {
            if let Some(decoded) = EvtUpdateRewardDuration::decode(data) {
                return Some(CpiEvent::EvtUpdateRewardDuration(decoded));
            }
        }

        {
            if let Some(decoded) = EvtUpdateRewardFunder::decode(data) {
                return Some(CpiEvent::EvtUpdateRewardFunder(decoded));
            }
        }

        {
            if let Some(decoded) = EvtWithdrawIneligibleReward::decode(data) {
                return Some(CpiEvent::EvtWithdrawIneligibleReward(decoded));
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
