pub mod close_permission_pda;
pub mod collect_creator_fee;
pub mod collect_fund_fee;
pub mod collect_protocol_fee;
pub mod cpi_event;
pub mod create_amm_config;
pub mod create_permission_pda;
pub mod deposit;
pub mod initialize;
pub mod initialize_with_permission;
pub mod swap_base_input;
pub mod swap_base_output;
pub mod update_amm_config;
pub mod update_pool_status;
pub mod withdraw;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{PROGRAM_ID, RaydiumCpmmDecoder};

pub use self::{
    close_permission_pda::*, collect_creator_fee::*, collect_fund_fee::*, collect_protocol_fee::*,
    cpi_event::*, create_amm_config::*, create_permission_pda::*, deposit::*, initialize::*,
    initialize_with_permission::*, swap_base_input::*, swap_base_output::*, update_amm_config::*,
    update_pool_status::*, withdraw::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum RaydiumCpmmInstruction {
    ClosePermissionPda {
        data: ClosePermissionPda,
        accounts: ClosePermissionPdaInstructionAccounts,
    },

    CollectCreatorFee {
        data: CollectCreatorFee,
        accounts: CollectCreatorFeeInstructionAccounts,
    },

    CollectFundFee {
        data: CollectFundFee,
        accounts: CollectFundFeeInstructionAccounts,
    },

    CollectProtocolFee {
        data: CollectProtocolFee,
        accounts: CollectProtocolFeeInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateAmmConfig {
        data: CreateAmmConfig,
        accounts: CreateAmmConfigInstructionAccounts,
    },

    CreatePermissionPda {
        data: CreatePermissionPda,
        accounts: CreatePermissionPdaInstructionAccounts,
    },

    Deposit {
        data: Deposit,
        accounts: DepositInstructionAccounts,
    },

    Initialize {
        data: Initialize,
        accounts: InitializeInstructionAccounts,
    },

    InitializeWithPermission {
        data: InitializeWithPermission,
        accounts: InitializeWithPermissionInstructionAccounts,
    },

    SwapBaseInput {
        data: SwapBaseInput,
        accounts: SwapBaseInputInstructionAccounts,
    },

    SwapBaseOutput {
        data: SwapBaseOutput,
        accounts: SwapBaseOutputInstructionAccounts,
    },

    UpdateAmmConfig {
        data: UpdateAmmConfig,
        accounts: UpdateAmmConfigInstructionAccounts,
    },

    UpdatePoolStatus {
        data: UpdatePoolStatus,
        accounts: UpdatePoolStatusInstructionAccounts,
    },

    Withdraw {
        data: Withdraw,
        accounts: WithdrawInstructionAccounts,
    },
}

impl InstructionDecoder for RaydiumCpmmDecoder {
    type InstructionType = RaydiumCpmmInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = ClosePermissionPda::decode(data)
                && let Some(accounts) = ClosePermissionPda::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::ClosePermissionPda { data, accounts });
            }
        }

        {
            if let Some(data) = CollectCreatorFee::decode(data)
                && let Some(accounts) = CollectCreatorFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CollectCreatorFee { data, accounts });
            }
        }

        {
            if let Some(data) = CollectFundFee::decode(data)
                && let Some(accounts) = CollectFundFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CollectFundFee { data, accounts });
            }
        }

        {
            if let Some(data) = CollectProtocolFee::decode(data)
                && let Some(accounts) = CollectProtocolFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CollectProtocolFee { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateAmmConfig::decode(data)
                && let Some(accounts) = CreateAmmConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CreateAmmConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePermissionPda::decode(data)
                && let Some(accounts) = CreatePermissionPda::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::CreatePermissionPda { data, accounts });
            }
        }

        {
            if let Some(data) = Deposit::decode(data)
                && let Some(accounts) = Deposit::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::Deposit { data, accounts });
            }
        }

        {
            if let Some(data) = Initialize::decode(data)
                && let Some(accounts) = Initialize::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::Initialize { data, accounts });
            }
        }

        {
            if let Some(data) = InitializeWithPermission::decode(data)
                && let Some(accounts) =
                    InitializeWithPermission::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::InitializeWithPermission { data, accounts });
            }
        }

        {
            if let Some(data) = SwapBaseInput::decode(data)
                && let Some(accounts) = SwapBaseInput::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::SwapBaseInput { data, accounts });
            }
        }

        {
            if let Some(data) = SwapBaseOutput::decode(data)
                && let Some(accounts) = SwapBaseOutput::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::SwapBaseOutput { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateAmmConfig::decode(data)
                && let Some(accounts) = UpdateAmmConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::UpdateAmmConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePoolStatus::decode(data)
                && let Some(accounts) = UpdatePoolStatus::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::UpdatePoolStatus { data, accounts });
            }
        }

        {
            if let Some(data) = Withdraw::decode(data)
                && let Some(accounts) = Withdraw::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumCpmmInstruction::Withdraw { data, accounts });
            }
        }

        None
    }
}
