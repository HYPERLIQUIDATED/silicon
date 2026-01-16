pub mod admin_set_creator;
pub mod admin_set_idl_authority;
pub mod admin_update_token_incentives;
pub mod buy;
pub mod buy_exact_sol_in;
pub mod claim_token_incentives;
pub mod close_user_volume_accumulator;
pub mod collect_creator_fee;
pub mod cpi_event;
pub mod create;
pub mod create_v2;
pub mod distribute_creator_fees;
pub mod extend_account;
pub mod get_minimum_distributable_fee;
pub mod init_user_volume_accumulator;
pub mod initialize;
pub mod migrate;
pub mod migrate_bonding_curve_creator;
pub mod sell;
pub mod set_creator;
pub mod set_mayhem_virtual_params;
pub mod set_metaplex_creator;
pub mod set_params;
pub mod set_reserved_fee_recipients;
pub mod sync_user_volume_accumulator;
pub mod toggle_create_v2;
pub mod toggle_mayhem_mode;
pub mod update_global_authority;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{PROGRAM_ID, PumpFunDecoder};

pub use self::{
    admin_set_creator::*, admin_set_idl_authority::*, admin_update_token_incentives::*, buy::*,
    buy_exact_sol_in::*, claim_token_incentives::*, close_user_volume_accumulator::*,
    collect_creator_fee::*, cpi_event::*, create::*, create_v2::*, distribute_creator_fees::*,
    extend_account::*, get_minimum_distributable_fee::*, init_user_volume_accumulator::*,
    initialize::*, migrate::*, migrate_bonding_curve_creator::*, sell::*, set_creator::*,
    set_mayhem_virtual_params::*, set_metaplex_creator::*, set_params::*,
    set_reserved_fee_recipients::*, sync_user_volume_accumulator::*, toggle_create_v2::*,
    toggle_mayhem_mode::*, update_global_authority::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PumpFunInstruction {
    AdminSetCreator {
        data: AdminSetCreator,
        accounts: AdminSetCreatorInstructionAccounts,
    },

    AdminSetIdlAuthority {
        data: AdminSetIdlAuthority,
        accounts: AdminSetIdlAuthorityInstructionAccounts,
    },

    AdminUpdateTokenIncentives {
        data: AdminUpdateTokenIncentives,
        accounts: AdminUpdateTokenIncentivesInstructionAccounts,
    },

    Buy {
        data: Buy,
        accounts: BuyInstructionAccounts,
    },

    BuyExactSolIn {
        data: BuyExactSolIn,
        accounts: BuyExactSolInInstructionAccounts,
    },

    ClaimTokenIncentives {
        data: ClaimTokenIncentives,
        accounts: ClaimTokenIncentivesInstructionAccounts,
    },

    CloseUserVolumeAccumulator {
        data: CloseUserVolumeAccumulator,
        accounts: CloseUserVolumeAccumulatorInstructionAccounts,
    },

    CollectCreatorFee {
        data: CollectCreatorFee,
        accounts: CollectCreatorFeeInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    Create {
        data: Create,
        accounts: CreateInstructionAccounts,
    },

    CreateV2 {
        data: CreateV2,
        accounts: CreateV2InstructionAccounts,
    },

    DistributeCreatorFees {
        data: DistributeCreatorFees,
        accounts: DistributeCreatorFeesInstructionAccounts,
    },

    ExtendAccount {
        data: ExtendAccount,
        accounts: ExtendAccountInstructionAccounts,
    },

    GetMinimumDistributableFee {
        data: GetMinimumDistributableFee,
        accounts: GetMinimumDistributableFeeInstructionAccounts,
    },

    Initialize {
        data: Initialize,
        accounts: InitializeInstructionAccounts,
    },

    InitUserVolumeAccumulator {
        data: InitUserVolumeAccumulator,
        accounts: InitUserVolumeAccumulatorInstructionAccounts,
    },

    Migrate {
        data: Migrate,
        accounts: MigrateInstructionAccounts,
    },

    MigrateBondingCurveCreator {
        data: MigrateBondingCurveCreator,
        accounts: MigrateBondingCurveCreatorInstructionAccounts,
    },

    Sell {
        data: Sell,
        accounts: SellInstructionAccounts,
    },

    SetCreator {
        data: SetCreator,
        accounts: SetCreatorInstructionAccounts,
    },

    SetMayhemVirtualParams {
        data: SetMayhemVirtualParams,
        accounts: SetMayhemVirtualParamsInstructionAccounts,
    },

    SetMetaplexCreator {
        data: SetMetaplexCreator,
        accounts: SetMetaplexCreatorInstructionAccounts,
    },

    SetParams {
        data: SetParams,
        accounts: SetParamsInstructionAccounts,
    },

    SetReservedFeeRecipients {
        data: SetReservedFeeRecipients,
        accounts: SetReservedFeeRecipientsInstructionAccounts,
    },

    SyncUserVolumeAccumulator {
        data: SyncUserVolumeAccumulator,
        accounts: SyncUserVolumeAccumulatorInstructionAccounts,
    },

    ToggleCreateV2 {
        data: ToggleCreateV2,
        accounts: ToggleCreateV2InstructionAccounts,
    },

    ToggleMayhemMode {
        data: ToggleMayhemMode,
        accounts: ToggleMayhemModeInstructionAccounts,
    },

    UpdateGlobalAuthority {
        data: UpdateGlobalAuthority,
        accounts: UpdateGlobalAuthorityInstructionAccounts,
    },
}

impl InstructionDecoder for PumpFunDecoder {
    type InstructionType = PumpFunInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = AdminSetCreator::decode(data)
                && let Some(accounts) = AdminSetCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::AdminSetCreator { data, accounts });
            }
        }

        {
            if let Some(data) = AdminSetIdlAuthority::decode(data)
                && let Some(accounts) =
                    AdminSetIdlAuthority::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::AdminSetIdlAuthority { data, accounts });
            }
        }

        {
            if let Some(data) = AdminUpdateTokenIncentives::decode(data)
                && let Some(accounts) =
                    AdminUpdateTokenIncentives::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::AdminUpdateTokenIncentives { data, accounts });
            }
        }

        {
            if let Some(data) = Buy::decode(data)
                && let Some(accounts) = Buy::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::Buy { data, accounts });
            }
        }

        {
            if let Some(data) = BuyExactSolIn::decode(data)
                && let Some(accounts) = BuyExactSolIn::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::BuyExactSolIn { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimTokenIncentives::decode(data)
                && let Some(accounts) =
                    ClaimTokenIncentives::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::ClaimTokenIncentives { data, accounts });
            }
        }

        {
            if let Some(data) = CloseUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    CloseUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::CloseUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = CollectCreatorFee::decode(data)
                && let Some(accounts) = CollectCreatorFee::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::CollectCreatorFee { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = Create::decode(data)
                && let Some(accounts) = Create::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::Create { data, accounts });
            }
        }

        {
            if let Some(data) = CreateV2::decode(data)
                && let Some(accounts) = CreateV2::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::CreateV2 { data, accounts });
            }
        }

        {
            if let Some(data) = DistributeCreatorFees::decode(data)
                && let Some(accounts) =
                    DistributeCreatorFees::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::DistributeCreatorFees { data, accounts });
            }
        }

        {
            if let Some(data) = ExtendAccount::decode(data)
                && let Some(accounts) = ExtendAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::ExtendAccount { data, accounts });
            }
        }

        {
            if let Some(data) = GetMinimumDistributableFee::decode(data)
                && let Some(accounts) =
                    GetMinimumDistributableFee::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::GetMinimumDistributableFee { data, accounts });
            }
        }

        {
            if let Some(data) = Initialize::decode(data)
                && let Some(accounts) = Initialize::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::Initialize { data, accounts });
            }
        }

        {
            if let Some(data) = InitUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    InitUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::InitUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = Migrate::decode(data)
                && let Some(accounts) = Migrate::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::Migrate { data, accounts });
            }
        }

        {
            if let Some(data) = MigrateBondingCurveCreator::decode(data)
                && let Some(accounts) =
                    MigrateBondingCurveCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::MigrateBondingCurveCreator { data, accounts });
            }
        }

        {
            if let Some(data) = Sell::decode(data)
                && let Some(accounts) = Sell::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::Sell { data, accounts });
            }
        }

        {
            if let Some(data) = SetCreator::decode(data)
                && let Some(accounts) = SetCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SetCreator { data, accounts });
            }
        }

        {
            if let Some(data) = SetMayhemVirtualParams::decode(data)
                && let Some(accounts) =
                    SetMayhemVirtualParams::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SetMayhemVirtualParams { data, accounts });
            }
        }

        {
            if let Some(data) = SetMetaplexCreator::decode(data)
                && let Some(accounts) = SetMetaplexCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SetMetaplexCreator { data, accounts });
            }
        }

        {
            if let Some(data) = SetParams::decode(data)
                && let Some(accounts) = SetParams::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SetParams { data, accounts });
            }
        }

        {
            if let Some(data) = SetReservedFeeRecipients::decode(data)
                && let Some(accounts) =
                    SetReservedFeeRecipients::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SetReservedFeeRecipients { data, accounts });
            }
        }

        {
            if let Some(data) = SyncUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    SyncUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::SyncUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = ToggleCreateV2::decode(data)
                && let Some(accounts) = ToggleCreateV2::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::ToggleCreateV2 { data, accounts });
            }
        }

        {
            if let Some(data) = ToggleMayhemMode::decode(data)
                && let Some(accounts) = ToggleMayhemMode::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::ToggleMayhemMode { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateGlobalAuthority::decode(data)
                && let Some(accounts) =
                    UpdateGlobalAuthority::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpFunInstruction::UpdateGlobalAuthority { data, accounts });
            }
        }

        None
    }
}
