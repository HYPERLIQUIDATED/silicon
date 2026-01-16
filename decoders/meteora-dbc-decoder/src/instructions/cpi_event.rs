use borsh::{BorshDeserialize, BorshSerialize};
use silicon_core::{account_utils::next_account, deserialize::ArrangeAccounts};
use solana_address::Address;
use solana_instruction::account_meta::AccountMeta;

use crate::events::{
    EvtClaimCreatorTradingFee, EvtClaimPoolCreationFee, EvtClaimProtocolFee,
    EvtClaimProtocolLiquidityMigrationFee, EvtClaimTradingFee, EvtCloseClaimFeeOperator,
    EvtCreateClaimFeeOperator, EvtCreateConfig, EvtCreateConfigV2,
    EvtCreateMeteoraMigrationMetadata, EvtCreatorWithdrawSurplus, EvtCurveComplete,
    EvtInitializePool, EvtPartnerClaimPoolCreationFee, EvtPartnerMetadata,
    EvtPartnerWithdrawMigrationFee, EvtPartnerWithdrawSurplus, EvtSwap, EvtSwap2,
    EvtUpdatePoolCreator, EvtVirtualPoolMetadata, EvtWithdrawLeftover, EvtWithdrawMigrationFee,
};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum CpiEvent {
    EvtClaimCreatorTradingFee(EvtClaimCreatorTradingFee),
    EvtClaimPoolCreationFee(EvtClaimPoolCreationFee),
    EvtClaimProtocolFee(EvtClaimProtocolFee),
    EvtClaimProtocolLiquidityMigrationFee(EvtClaimProtocolLiquidityMigrationFee),
    EvtClaimTradingFee(EvtClaimTradingFee),
    EvtCloseClaimFeeOperator(EvtCloseClaimFeeOperator),
    EvtCreateClaimFeeOperator(EvtCreateClaimFeeOperator),
    EvtCreateConfig(EvtCreateConfig),
    EvtCreateConfigV2(EvtCreateConfigV2),
    EvtCreateMeteoraMigrationMetadata(EvtCreateMeteoraMigrationMetadata),
    EvtCreatorWithdrawSurplus(EvtCreatorWithdrawSurplus),
    EvtCurveComplete(EvtCurveComplete),
    EvtInitializePool(EvtInitializePool),
    EvtPartnerClaimPoolCreationFee(EvtPartnerClaimPoolCreationFee),
    EvtPartnerMetadata(EvtPartnerMetadata),
    EvtPartnerWithdrawMigrationFee(EvtPartnerWithdrawMigrationFee),
    EvtPartnerWithdrawSurplus(EvtPartnerWithdrawSurplus),
    EvtSwap(EvtSwap),
    EvtSwap2(EvtSwap2),
    EvtUpdatePoolCreator(EvtUpdatePoolCreator),
    EvtVirtualPoolMetadata(EvtVirtualPoolMetadata),
    EvtWithdrawLeftover(EvtWithdrawLeftover),
    EvtWithdrawMigrationFee(EvtWithdrawMigrationFee),
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
            if let Some(decoded) = EvtClaimCreatorTradingFee::decode(data) {
                return Some(CpiEvent::EvtClaimCreatorTradingFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimPoolCreationFee::decode(data) {
                return Some(CpiEvent::EvtClaimPoolCreationFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimProtocolFee::decode(data) {
                return Some(CpiEvent::EvtClaimProtocolFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimProtocolLiquidityMigrationFee::decode(data) {
                return Some(CpiEvent::EvtClaimProtocolLiquidityMigrationFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtClaimTradingFee::decode(data) {
                return Some(CpiEvent::EvtClaimTradingFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCloseClaimFeeOperator::decode(data) {
                return Some(CpiEvent::EvtCloseClaimFeeOperator(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateClaimFeeOperator::decode(data) {
                return Some(CpiEvent::EvtCreateClaimFeeOperator(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateConfig::decode(data) {
                return Some(CpiEvent::EvtCreateConfig(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateConfigV2::decode(data) {
                return Some(CpiEvent::EvtCreateConfigV2(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreateMeteoraMigrationMetadata::decode(data) {
                return Some(CpiEvent::EvtCreateMeteoraMigrationMetadata(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCreatorWithdrawSurplus::decode(data) {
                return Some(CpiEvent::EvtCreatorWithdrawSurplus(decoded));
            }
        }

        {
            if let Some(decoded) = EvtCurveComplete::decode(data) {
                return Some(CpiEvent::EvtCurveComplete(decoded));
            }
        }

        {
            if let Some(decoded) = EvtInitializePool::decode(data) {
                return Some(CpiEvent::EvtInitializePool(decoded));
            }
        }

        {
            if let Some(decoded) = EvtPartnerClaimPoolCreationFee::decode(data) {
                return Some(CpiEvent::EvtPartnerClaimPoolCreationFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtPartnerMetadata::decode(data) {
                return Some(CpiEvent::EvtPartnerMetadata(decoded));
            }
        }

        {
            if let Some(decoded) = EvtPartnerWithdrawMigrationFee::decode(data) {
                return Some(CpiEvent::EvtPartnerWithdrawMigrationFee(decoded));
            }
        }

        {
            if let Some(decoded) = EvtPartnerWithdrawSurplus::decode(data) {
                return Some(CpiEvent::EvtPartnerWithdrawSurplus(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSwap::decode(data) {
                return Some(CpiEvent::EvtSwap(decoded));
            }
        }

        {
            if let Some(decoded) = EvtSwap2::decode(data) {
                return Some(CpiEvent::EvtSwap2(decoded));
            }
        }

        {
            if let Some(decoded) = EvtUpdatePoolCreator::decode(data) {
                return Some(CpiEvent::EvtUpdatePoolCreator(decoded));
            }
        }

        {
            if let Some(decoded) = EvtVirtualPoolMetadata::decode(data) {
                return Some(CpiEvent::EvtVirtualPoolMetadata(decoded));
            }
        }

        {
            if let Some(decoded) = EvtWithdrawLeftover::decode(data) {
                return Some(CpiEvent::EvtWithdrawLeftover(decoded));
            }
        }

        {
            if let Some(decoded) = EvtWithdrawMigrationFee::decode(data) {
                return Some(CpiEvent::EvtWithdrawMigrationFee(decoded));
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
