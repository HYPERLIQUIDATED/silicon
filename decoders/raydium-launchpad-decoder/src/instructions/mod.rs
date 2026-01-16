pub mod buy_exact_in;
pub mod buy_exact_out;
pub mod claim_creator_fee;
pub mod claim_platform_fee;
pub mod claim_platform_fee_from_vault;
pub mod claim_vested_token;
pub mod collect_fee;
pub mod collect_migrate_fee;
pub mod cpi_event;
pub mod create_config;
pub mod create_platform_config;
pub mod create_platform_vesting_account;
pub mod create_vesting_account;
pub mod initialize;
pub mod initialize_v2;
pub mod initialize_with_token2022;
pub mod migrate_to_amm;
pub mod migrate_to_cpswap;
pub mod remove_platform_curve_param;
pub mod sell_exact_in;
pub mod sell_exact_out;
pub mod update_config;
pub mod update_platform_config;
pub mod update_platform_curve_param;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{PROGRAM_ID, RaydiumLaunchpadDecoder};

pub use self::{
    buy_exact_in::*, buy_exact_out::*, claim_creator_fee::*, claim_platform_fee::*,
    claim_platform_fee_from_vault::*, claim_vested_token::*, collect_fee::*,
    collect_migrate_fee::*, cpi_event::*, create_config::*, create_platform_config::*,
    create_platform_vesting_account::*, create_vesting_account::*, initialize::*, initialize_v2::*,
    initialize_with_token2022::*, migrate_to_amm::*, migrate_to_cpswap::*,
    remove_platform_curve_param::*, sell_exact_in::*, sell_exact_out::*, update_config::*,
    update_platform_config::*, update_platform_curve_param::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum RaydiumLaunchpadInstruction {
    BuyExactIn {
        data: BuyExactIn,
        accounts: BuyExactInInstructionAccounts,
    },

    BuyExactOut {
        data: BuyExactOut,
        accounts: BuyExactOutInstructionAccounts,
    },

    ClaimCreatorFee {
        data: ClaimCreatorFee,
        accounts: ClaimCreatorFeeInstructionAccounts,
    },

    ClaimPlatformFee {
        data: ClaimPlatformFee,
        accounts: ClaimPlatformFeeInstructionAccounts,
    },

    ClaimPlatformFeeFromVault {
        data: ClaimPlatformFeeFromVault,
        accounts: ClaimPlatformFeeFromVaultInstructionAccounts,
    },

    ClaimVestedToken {
        data: ClaimVestedToken,
        accounts: ClaimVestedTokenInstructionAccounts,
    },

    CollectFee {
        data: CollectFee,
        accounts: CollectFeeInstructionAccounts,
    },

    CollectMigrateFee {
        data: CollectMigrateFee,
        accounts: CollectMigrateFeeInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateConfig {
        data: CreateConfig,
        accounts: CreateConfigInstructionAccounts,
    },

    CreatePlatformConfig {
        data: CreatePlatformConfig,
        accounts: CreatePlatformConfigInstructionAccounts,
    },

    CreatePlatformVestingAccount {
        data: CreatePlatformVestingAccount,
        accounts: CreatePlatformVestingAccountInstructionAccounts,
    },

    CreateVestingAccount {
        data: CreateVestingAccount,
        accounts: CreateVestingAccountInstructionAccounts,
    },

    Initialize {
        data: Initialize,
        accounts: InitializeInstructionAccounts,
    },

    InitializeV2 {
        data: InitializeV2,
        accounts: InitializeV2InstructionAccounts,
    },

    InitializeWithToken2022 {
        data: InitializeWithToken2022,
        accounts: InitializeWithToken2022InstructionAccounts,
    },

    MigrateToAmm {
        data: MigrateToAmm,
        accounts: MigrateToAmmInstructionAccounts,
    },

    MigrateToCpswap {
        data: MigrateToCpswap,
        accounts: MigrateToCpswapInstructionAccounts,
    },

    RemovePlatformCurveParam {
        data: RemovePlatformCurveParam,
        accounts: RemovePlatformCurveParamInstructionAccounts,
    },

    SellExactIn {
        data: SellExactIn,
        accounts: SellExactInInstructionAccounts,
    },

    SellExactOut {
        data: SellExactOut,
        accounts: SellExactOutInstructionAccounts,
    },

    UpdateConfig {
        data: UpdateConfig,
        accounts: UpdateConfigInstructionAccounts,
    },

    UpdatePlatformConfig {
        data: UpdatePlatformConfig,
        accounts: UpdatePlatformConfigInstructionAccounts,
    },

    UpdatePlatformCurveParam {
        data: UpdatePlatformCurveParam,
        accounts: UpdatePlatformCurveParamInstructionAccounts,
    },
}

impl InstructionDecoder for RaydiumLaunchpadDecoder {
    type InstructionType = RaydiumLaunchpadInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = BuyExactIn::decode(data)
                && let Some(accounts) = BuyExactIn::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::BuyExactIn { data, accounts });
            }
        }

        {
            if let Some(data) = BuyExactOut::decode(data)
                && let Some(accounts) = BuyExactOut::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::BuyExactOut { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimCreatorFee::decode(data)
                && let Some(accounts) = ClaimCreatorFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::ClaimCreatorFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPlatformFee::decode(data)
                && let Some(accounts) = ClaimPlatformFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::ClaimPlatformFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPlatformFeeFromVault::decode(data)
                && let Some(accounts) =
                    ClaimPlatformFeeFromVault::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::ClaimPlatformFeeFromVault {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = ClaimVestedToken::decode(data)
                && let Some(accounts) = ClaimVestedToken::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::ClaimVestedToken { data, accounts });
            }
        }

        {
            if let Some(data) = CollectFee::decode(data)
                && let Some(accounts) = CollectFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CollectFee { data, accounts });
            }
        }

        {
            if let Some(data) = CollectMigrateFee::decode(data)
                && let Some(accounts) = CollectMigrateFee::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CollectMigrateFee { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateConfig::decode(data)
                && let Some(accounts) = CreateConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CreateConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePlatformConfig::decode(data)
                && let Some(accounts) =
                    CreatePlatformConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CreatePlatformConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePlatformVestingAccount::decode(data)
                && let Some(accounts) =
                    CreatePlatformVestingAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CreatePlatformVestingAccount {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = CreateVestingAccount::decode(data)
                && let Some(accounts) =
                    CreateVestingAccount::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::CreateVestingAccount { data, accounts });
            }
        }

        {
            if let Some(data) = Initialize::decode(data)
                && let Some(accounts) = Initialize::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::Initialize { data, accounts });
            }
        }

        {
            if let Some(data) = InitializeV2::decode(data)
                && let Some(accounts) = InitializeV2::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::InitializeV2 { data, accounts });
            }
        }

        {
            if let Some(data) = InitializeWithToken2022::decode(data)
                && let Some(accounts) =
                    InitializeWithToken2022::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::InitializeWithToken2022 {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = MigrateToAmm::decode(data)
                && let Some(accounts) = MigrateToAmm::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::MigrateToAmm { data, accounts });
            }
        }

        {
            if let Some(data) = MigrateToCpswap::decode(data)
                && let Some(accounts) = MigrateToCpswap::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::MigrateToCpswap { data, accounts });
            }
        }

        {
            if let Some(data) = RemovePlatformCurveParam::decode(data)
                && let Some(accounts) =
                    RemovePlatformCurveParam::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::RemovePlatformCurveParam {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = SellExactIn::decode(data)
                && let Some(accounts) = SellExactIn::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::SellExactIn { data, accounts });
            }
        }

        {
            if let Some(data) = SellExactOut::decode(data)
                && let Some(accounts) = SellExactOut::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::SellExactOut { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateConfig::decode(data)
                && let Some(accounts) = UpdateConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::UpdateConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePlatformConfig::decode(data)
                && let Some(accounts) =
                    UpdatePlatformConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::UpdatePlatformConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePlatformCurveParam::decode(data)
                && let Some(accounts) =
                    UpdatePlatformCurveParam::arrange_accounts(&instruction.accounts)
            {
                return Some(RaydiumLaunchpadInstruction::UpdatePlatformCurveParam {
                    data,
                    accounts,
                });
            }
        }

        None
    }
}
