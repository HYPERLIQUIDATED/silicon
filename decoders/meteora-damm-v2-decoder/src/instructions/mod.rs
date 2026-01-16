pub mod add_liquidity;
pub mod claim_partner_fee;
pub mod claim_position_fee;
pub mod claim_protocol_fee;
pub mod claim_reward;
pub mod close_config;
pub mod close_operator_account;
pub mod close_position;
pub mod close_token_badge;
pub mod cpi_event;
pub mod create_config;
pub mod create_dynamic_config;
pub mod create_operator_account;
pub mod create_position;
pub mod create_token_badge;
pub mod dummy_ix;
pub mod fix_config_fee_params;
pub mod fix_pool_fee_params;
pub mod fund_reward;
pub mod initialize_customizable_pool;
pub mod initialize_pool;
pub mod initialize_pool_with_dynamic_config;
pub mod initialize_reward;
pub mod lock_inner_position;
pub mod lock_position;
pub mod permanent_lock_position;
pub mod refresh_vesting;
pub mod remove_all_liquidity;
pub mod remove_liquidity;
pub mod set_pool_status;
pub mod split_position;
pub mod split_position2;
pub mod swap;
pub mod swap2;
pub mod update_pool_fees;
pub mod update_reward_duration;
pub mod update_reward_funder;
pub mod withdraw_ineligible_reward;
pub mod zap_protocol_fee;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{MeteoraDammV2Decoder, PROGRAM_ID};

pub use self::{
    add_liquidity::*, claim_partner_fee::*, claim_position_fee::*, claim_protocol_fee::*,
    claim_reward::*, close_config::*, close_operator_account::*, close_position::*,
    close_token_badge::*, cpi_event::*, create_config::*, create_dynamic_config::*,
    create_operator_account::*, create_position::*, create_token_badge::*, dummy_ix::*,
    fix_config_fee_params::*, fix_pool_fee_params::*, fund_reward::*,
    initialize_customizable_pool::*, initialize_pool::*, initialize_pool_with_dynamic_config::*,
    initialize_reward::*, lock_inner_position::*, lock_position::*, permanent_lock_position::*,
    refresh_vesting::*, remove_all_liquidity::*, remove_liquidity::*, set_pool_status::*,
    split_position::*, split_position2::*, swap::*, swap2::*, update_pool_fees::*,
    update_reward_duration::*, update_reward_funder::*, withdraw_ineligible_reward::*,
    zap_protocol_fee::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MeteoraDammV2Instruction {
    AddLiquidity {
        data: AddLiquidity,
        accounts: AddLiquidityInstructionAccounts,
    },

    ClaimPartnerFee {
        data: ClaimPartnerFee,
        accounts: ClaimPartnerFeeInstructionAccounts,
    },

    ClaimPositionFee {
        data: ClaimPositionFee,
        accounts: ClaimPositionFeeInstructionAccounts,
    },

    ClaimProtocolFee {
        data: ClaimProtocolFee,
        accounts: ClaimProtocolFeeInstructionAccounts,
    },

    ClaimReward {
        data: ClaimReward,
        accounts: ClaimRewardInstructionAccounts,
    },

    CloseConfig {
        data: CloseConfig,
        accounts: CloseConfigInstructionAccounts,
    },

    CloseOperatorAccount {
        data: CloseOperatorAccount,
        accounts: CloseOperatorAccountInstructionAccounts,
    },

    ClosePosition {
        data: ClosePosition,
        accounts: ClosePositionInstructionAccounts,
    },

    CloseTokenBadge {
        data: CloseTokenBadge,
        accounts: CloseTokenBadgeInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateConfig {
        data: CreateConfig,
        accounts: CreateConfigInstructionAccounts,
    },

    CreateDynamicConfig {
        data: CreateDynamicConfig,
        accounts: CreateDynamicConfigInstructionAccounts,
    },

    CreateOperatorAccount {
        data: CreateOperatorAccount,
        accounts: CreateOperatorAccountInstructionAccounts,
    },

    CreatePosition {
        data: CreatePosition,
        accounts: CreatePositionInstructionAccounts,
    },

    CreateTokenBadge {
        data: CreateTokenBadge,
        accounts: CreateTokenBadgeInstructionAccounts,
    },

    DummyIx {
        data: DummyIx,
        accounts: DummyIxInstructionAccounts,
    },

    FixConfigFeeParams {
        data: FixConfigFeeParams,
        accounts: FixConfigFeeParamsInstructionAccounts,
    },

    FixPoolFeeParams {
        data: FixPoolFeeParams,
        accounts: FixPoolFeeParamsInstructionAccounts,
    },

    FundReward {
        data: FundReward,
        accounts: FundRewardInstructionAccounts,
    },

    InitializeCustomizablePool {
        data: InitializeCustomizablePool,
        accounts: InitializeCustomizablePoolInstructionAccounts,
    },

    InitializePool {
        data: InitializePool,
        accounts: InitializePoolInstructionAccounts,
    },

    InitializePoolWithDynamicConfig {
        data: InitializePoolWithDynamicConfig,
        accounts: InitializePoolWithDynamicConfigInstructionAccounts,
    },

    InitializeReward {
        data: InitializeReward,
        accounts: InitializeRewardInstructionAccounts,
    },

    LockInnerPosition {
        data: LockInnerPosition,
        accounts: LockInnerPositionInstructionAccounts,
    },

    LockPosition {
        data: LockPosition,
        accounts: LockPositionInstructionAccounts,
    },

    PermanentLockPosition {
        data: PermanentLockPosition,
        accounts: PermanentLockPositionInstructionAccounts,
    },

    RefreshVesting {
        data: RefreshVesting,
        accounts: RefreshVestingInstructionAccounts,
    },

    RemoveAllLiquidity {
        data: RemoveAllLiquidity,
        accounts: RemoveAllLiquidityInstructionAccounts,
    },

    RemoveLiquidity {
        data: RemoveLiquidity,
        accounts: RemoveLiquidityInstructionAccounts,
    },

    SetPoolStatus {
        data: SetPoolStatus,
        accounts: SetPoolStatusInstructionAccounts,
    },

    SplitPosition {
        data: SplitPosition,
        accounts: SplitPositionInstructionAccounts,
    },

    SplitPosition2 {
        data: SplitPosition2,
        accounts: SplitPosition2InstructionAccounts,
    },

    Swap {
        data: Swap,
        accounts: SwapInstructionAccounts,
    },

    Swap2 {
        data: Swap2,
        accounts: Swap2InstructionAccounts,
    },

    UpdatePoolFees {
        data: UpdatePoolFees,
        accounts: UpdatePoolFeesInstructionAccounts,
    },

    UpdateRewardDuration {
        data: UpdateRewardDuration,
        accounts: UpdateRewardDurationInstructionAccounts,
    },

    UpdateRewardFunder {
        data: UpdateRewardFunder,
        accounts: UpdateRewardFunderInstructionAccounts,
    },

    WithdrawIneligibleReward {
        data: WithdrawIneligibleReward,
        accounts: WithdrawIneligibleRewardInstructionAccounts,
    },

    ZapProtocolFee {
        data: ZapProtocolFee,
        accounts: ZapProtocolFeeInstructionAccounts,
    },
}

impl InstructionDecoder for MeteoraDammV2Decoder {
    type InstructionType = MeteoraDammV2Instruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = AddLiquidity::decode(data)
                && let Some(accounts) = AddLiquidity::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::AddLiquidity { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPartnerFee::decode(data)
                && let Some(accounts) = ClaimPartnerFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ClaimPartnerFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPositionFee::decode(data)
                && let Some(accounts) = ClaimPositionFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ClaimPositionFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimProtocolFee::decode(data)
                && let Some(accounts) = ClaimProtocolFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ClaimProtocolFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimReward::decode(data)
                && let Some(accounts) = ClaimReward::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ClaimReward { data, accounts });
            }
        }

        {
            if let Some(data) = CloseConfig::decode(data)
                && let Some(accounts) = CloseConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CloseConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CloseOperatorAccount::decode(data)
                && let Some(accounts) =
                    CloseOperatorAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CloseOperatorAccount { data, accounts });
            }
        }

        {
            if let Some(data) = ClosePosition::decode(data)
                && let Some(accounts) = ClosePosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ClosePosition { data, accounts });
            }
        }

        {
            if let Some(data) = CloseTokenBadge::decode(data)
                && let Some(accounts) = CloseTokenBadge::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CloseTokenBadge { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateConfig::decode(data)
                && let Some(accounts) = CreateConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CreateConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreateDynamicConfig::decode(data)
                && let Some(accounts) = CreateDynamicConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CreateDynamicConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreateOperatorAccount::decode(data)
                && let Some(accounts) =
                    CreateOperatorAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CreateOperatorAccount { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePosition::decode(data)
                && let Some(accounts) = CreatePosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CreatePosition { data, accounts });
            }
        }

        {
            if let Some(data) = CreateTokenBadge::decode(data)
                && let Some(accounts) = CreateTokenBadge::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::CreateTokenBadge { data, accounts });
            }
        }

        {
            if let Some(data) = DummyIx::decode(data)
                && let Some(accounts) = DummyIx::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::DummyIx { data, accounts });
            }
        }

        {
            if let Some(data) = FixConfigFeeParams::decode(data)
                && let Some(accounts) = FixConfigFeeParams::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::FixConfigFeeParams { data, accounts });
            }
        }

        {
            if let Some(data) = FixPoolFeeParams::decode(data)
                && let Some(accounts) = FixPoolFeeParams::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::FixPoolFeeParams { data, accounts });
            }
        }

        {
            if let Some(data) = FundReward::decode(data)
                && let Some(accounts) = FundReward::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::FundReward { data, accounts });
            }
        }

        {
            if let Some(data) = InitializeCustomizablePool::decode(data)
                && let Some(accounts) =
                    InitializeCustomizablePool::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::InitializeCustomizablePool {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = InitializePool::decode(data)
                && let Some(accounts) = InitializePool::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::InitializePool { data, accounts });
            }
        }

        {
            if let Some(data) = InitializePoolWithDynamicConfig::decode(data)
                && let Some(accounts) =
                    InitializePoolWithDynamicConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::InitializePoolWithDynamicConfig {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = InitializeReward::decode(data)
                && let Some(accounts) = InitializeReward::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::InitializeReward { data, accounts });
            }
        }

        {
            if let Some(data) = LockInnerPosition::decode(data)
                && let Some(accounts) = LockInnerPosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::LockInnerPosition { data, accounts });
            }
        }

        {
            if let Some(data) = LockPosition::decode(data)
                && let Some(accounts) = LockPosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::LockPosition { data, accounts });
            }
        }

        {
            if let Some(data) = PermanentLockPosition::decode(data)
                && let Some(accounts) =
                    PermanentLockPosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::PermanentLockPosition { data, accounts });
            }
        }

        {
            if let Some(data) = RefreshVesting::decode(data)
                && let Some(accounts) = RefreshVesting::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::RefreshVesting { data, accounts });
            }
        }

        {
            if let Some(data) = RemoveAllLiquidity::decode(data)
                && let Some(accounts) = RemoveAllLiquidity::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::RemoveAllLiquidity { data, accounts });
            }
        }

        {
            if let Some(data) = RemoveLiquidity::decode(data)
                && let Some(accounts) = RemoveLiquidity::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::RemoveLiquidity { data, accounts });
            }
        }

        {
            if let Some(data) = SetPoolStatus::decode(data)
                && let Some(accounts) = SetPoolStatus::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::SetPoolStatus { data, accounts });
            }
        }

        {
            if let Some(data) = SplitPosition::decode(data)
                && let Some(accounts) = SplitPosition::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::SplitPosition { data, accounts });
            }
        }

        {
            if let Some(data) = SplitPosition2::decode(data)
                && let Some(accounts) = SplitPosition2::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::SplitPosition2 { data, accounts });
            }
        }

        {
            if let Some(data) = Swap::decode(data)
                && let Some(accounts) = Swap::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::Swap { data, accounts });
            }
        }

        {
            if let Some(data) = Swap2::decode(data)
                && let Some(accounts) = Swap2::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::Swap2 { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePoolFees::decode(data)
                && let Some(accounts) = UpdatePoolFees::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::UpdatePoolFees { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateRewardDuration::decode(data)
                && let Some(accounts) =
                    UpdateRewardDuration::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::UpdateRewardDuration { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateRewardFunder::decode(data)
                && let Some(accounts) = UpdateRewardFunder::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::UpdateRewardFunder { data, accounts });
            }
        }

        {
            if let Some(data) = WithdrawIneligibleReward::decode(data)
                && let Some(accounts) =
                    WithdrawIneligibleReward::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::WithdrawIneligibleReward { data, accounts });
            }
        }

        {
            if let Some(data) = ZapProtocolFee::decode(data)
                && let Some(accounts) = ZapProtocolFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDammV2Instruction::ZapProtocolFee { data, accounts });
            }
        }

        None
    }
}
