pub mod admin_set_coin_creator;
pub mod admin_update_token_incentives;
pub mod buy;
pub mod buy_exact_quote_in;
pub mod claim_token_incentives;
pub mod close_user_volume_accumulator;
pub mod collect_coin_creator_fee;
pub mod cpi_event;
pub mod create_config;
pub mod create_pool;
pub mod deposit;
pub mod disable;
pub mod extend_account;
pub mod init_user_volume_accumulator;
pub mod migrate_pool_coin_creator;
pub mod sell;
pub mod set_coin_creator;
pub mod set_reserved_fee_recipients;
pub mod sync_user_volume_accumulator;
pub mod toggle_mayhem_mode;
pub mod transfer_creator_fees_to_pump;
pub mod update_admin;
pub mod update_fee_config;
pub mod withdraw;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{PROGRAM_ID, PumpAmmDecoder};

pub use self::{
    admin_set_coin_creator::*, admin_update_token_incentives::*, buy::*, buy_exact_quote_in::*,
    claim_token_incentives::*, close_user_volume_accumulator::*, collect_coin_creator_fee::*,
    cpi_event::*, create_config::*, create_pool::*, deposit::*, disable::*, extend_account::*,
    init_user_volume_accumulator::*, migrate_pool_coin_creator::*, sell::*, set_coin_creator::*,
    set_reserved_fee_recipients::*, sync_user_volume_accumulator::*, toggle_mayhem_mode::*,
    transfer_creator_fees_to_pump::*, update_admin::*, update_fee_config::*, withdraw::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PumpAmmInstruction {
    AdminSetCoinCreator {
        data: AdminSetCoinCreator,
        accounts: AdminSetCoinCreatorInstructionAccounts,
    },

    AdminUpdateTokenIncentives {
        data: AdminUpdateTokenIncentives,
        accounts: AdminUpdateTokenIncentivesInstructionAccounts,
    },

    Buy {
        data: Buy,
        accounts: BuyInstructionAccounts,
    },

    BuyExactQuoteIn {
        data: BuyExactQuoteIn,
        accounts: BuyExactQuoteInInstructionAccounts,
    },

    ClaimTokenIncentives {
        data: ClaimTokenIncentives,
        accounts: ClaimTokenIncentivesInstructionAccounts,
    },

    CloseUserVolumeAccumulator {
        data: CloseUserVolumeAccumulator,
        accounts: CloseUserVolumeAccumulatorInstructionAccounts,
    },

    CollectCoinCreatorFee {
        data: CollectCoinCreatorFee,
        accounts: CollectCoinCreatorFeeInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateConfig {
        data: CreateConfig,
        accounts: CreateConfigInstructionAccounts,
    },

    CreatePool {
        data: CreatePool,
        accounts: CreatePoolInstructionAccounts,
    },

    Deposit {
        data: Deposit,
        accounts: DepositInstructionAccounts,
    },

    Disable {
        data: Disable,
        accounts: DisableInstructionAccounts,
    },

    ExtendAccount {
        data: ExtendAccount,
        accounts: ExtendAccountInstructionAccounts,
    },

    InitUserVolumeAccumulator {
        data: InitUserVolumeAccumulator,
        accounts: InitUserVolumeAccumulatorInstructionAccounts,
    },

    MigratePoolCoinCreator {
        data: MigratePoolCoinCreator,
        accounts: MigratePoolCoinCreatorInstructionAccounts,
    },

    Sell {
        data: Sell,
        accounts: SellInstructionAccounts,
    },

    SetCoinCreator {
        data: SetCoinCreator,
        accounts: SetCoinCreatorInstructionAccounts,
    },

    SetReservedFeeRecipients {
        data: SetReservedFeeRecipients,
        accounts: SetReservedFeeRecipientsInstructionAccounts,
    },

    SyncUserVolumeAccumulator {
        data: SyncUserVolumeAccumulator,
        accounts: SyncUserVolumeAccumulatorInstructionAccounts,
    },

    ToggleMayhemMode {
        data: ToggleMayhemMode,
        accounts: ToggleMayhemModeInstructionAccounts,
    },

    TransferCreatorFeesToPump {
        data: TransferCreatorFeesToPump,
        accounts: TransferCreatorFeesToPumpInstructionAccounts,
    },

    UpdateAdmin {
        data: UpdateAdmin,
        accounts: UpdateAdminInstructionAccounts,
    },

    UpdateFeeConfig {
        data: UpdateFeeConfig,
        accounts: UpdateFeeConfigInstructionAccounts,
    },

    Withdraw {
        data: Withdraw,
        accounts: WithdrawInstructionAccounts,
    },
}

impl InstructionDecoder for PumpAmmDecoder {
    type InstructionType = PumpAmmInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = AdminSetCoinCreator::decode(data)
                && let Some(accounts) = AdminSetCoinCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::AdminSetCoinCreator { data, accounts });
            }
        }

        {
            if let Some(data) = AdminUpdateTokenIncentives::decode(data)
                && let Some(accounts) =
                    AdminUpdateTokenIncentives::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::AdminUpdateTokenIncentives { data, accounts });
            }
        }

        {
            if let Some(data) = Buy::decode(data)
                && let Some(accounts) = Buy::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::Buy { data, accounts });
            }
        }

        {
            if let Some(data) = BuyExactQuoteIn::decode(data)
                && let Some(accounts) = BuyExactQuoteIn::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::BuyExactQuoteIn { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimTokenIncentives::decode(data)
                && let Some(accounts) =
                    ClaimTokenIncentives::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::ClaimTokenIncentives { data, accounts });
            }
        }

        {
            if let Some(data) = CloseUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    CloseUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::CloseUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = CollectCoinCreatorFee::decode(data)
                && let Some(accounts) =
                    CollectCoinCreatorFee::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::CollectCoinCreatorFee { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateConfig::decode(data)
                && let Some(accounts) = CreateConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::CreateConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePool::decode(data)
                && let Some(accounts) = CreatePool::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::CreatePool { data, accounts });
            }
        }

        {
            if let Some(data) = Deposit::decode(data)
                && let Some(accounts) = Deposit::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::Deposit { data, accounts });
            }
        }

        {
            if let Some(data) = Disable::decode(data)
                && let Some(accounts) = Disable::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::Disable { data, accounts });
            }
        }

        {
            if let Some(data) = ExtendAccount::decode(data)
                && let Some(accounts) = ExtendAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::ExtendAccount { data, accounts });
            }
        }

        {
            if let Some(data) = InitUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    InitUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::InitUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = MigratePoolCoinCreator::decode(data)
                && let Some(accounts) =
                    MigratePoolCoinCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::MigratePoolCoinCreator { data, accounts });
            }
        }

        {
            if let Some(data) = Sell::decode(data)
                && let Some(accounts) = Sell::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::Sell { data, accounts });
            }
        }

        {
            if let Some(data) = SetCoinCreator::decode(data)
                && let Some(accounts) = SetCoinCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::SetCoinCreator { data, accounts });
            }
        }

        {
            if let Some(data) = SetReservedFeeRecipients::decode(data)
                && let Some(accounts) =
                    SetReservedFeeRecipients::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::SetReservedFeeRecipients { data, accounts });
            }
        }

        {
            if let Some(data) = SyncUserVolumeAccumulator::decode(data)
                && let Some(accounts) =
                    SyncUserVolumeAccumulator::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::SyncUserVolumeAccumulator { data, accounts });
            }
        }

        {
            if let Some(data) = ToggleMayhemMode::decode(data)
                && let Some(accounts) = ToggleMayhemMode::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::ToggleMayhemMode { data, accounts });
            }
        }

        {
            if let Some(data) = TransferCreatorFeesToPump::decode(data)
                && let Some(accounts) =
                    TransferCreatorFeesToPump::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::TransferCreatorFeesToPump { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateAdmin::decode(data)
                && let Some(accounts) = UpdateAdmin::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::UpdateAdmin { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateFeeConfig::decode(data)
                && let Some(accounts) = UpdateFeeConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::UpdateFeeConfig { data, accounts });
            }
        }

        {
            if let Some(data) = Withdraw::decode(data)
                && let Some(accounts) = Withdraw::arrange_accounts(&instruction.accounts)
            {
                return Some(PumpAmmInstruction::Withdraw { data, accounts });
            }
        }

        None
    }
}
