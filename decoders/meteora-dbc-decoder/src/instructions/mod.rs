pub mod claim_creator_trading_fee;
pub mod claim_partner_pool_creation_fee;
pub mod claim_protocol_fee;
pub mod claim_protocol_pool_creation_fee;
pub mod claim_trading_fee;
pub mod close_claim_protocol_fee_operator;
pub mod cpi_event;
pub mod create_claim_protocol_fee_operator;
pub mod create_config;
pub mod create_locker;
pub mod create_partner_metadata;
pub mod create_virtual_pool_metadata;
pub mod creator_withdraw_surplus;
pub mod initialize_virtual_pool_with_spl_token;
pub mod initialize_virtual_pool_with_token2022;
pub mod migrate_meteora_damm;
pub mod migrate_meteora_damm_claim_lp_token;
pub mod migrate_meteora_damm_lock_lp_token;
pub mod migration_damm_v2;
pub mod migration_damm_v2_create_metadata;
pub mod migration_meteora_damm_create_metadata;
pub mod partner_withdraw_surplus;
pub mod swap;
pub mod swap2;
pub mod transfer_pool_creator;
pub mod withdraw_leftover;
pub mod withdraw_migration_fee;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{MeteoraDbcDecoder, PROGRAM_ID};

pub use self::{
    claim_creator_trading_fee::*, claim_partner_pool_creation_fee::*, claim_protocol_fee::*,
    claim_protocol_pool_creation_fee::*, claim_trading_fee::*,
    close_claim_protocol_fee_operator::*, cpi_event::*, create_claim_protocol_fee_operator::*,
    create_config::*, create_locker::*, create_partner_metadata::*,
    create_virtual_pool_metadata::*, creator_withdraw_surplus::*,
    initialize_virtual_pool_with_spl_token::*, initialize_virtual_pool_with_token2022::*,
    migrate_meteora_damm::*, migrate_meteora_damm_claim_lp_token::*,
    migrate_meteora_damm_lock_lp_token::*, migration_damm_v2::*,
    migration_damm_v2_create_metadata::*, migration_meteora_damm_create_metadata::*,
    partner_withdraw_surplus::*, swap::*, swap2::*, transfer_pool_creator::*, withdraw_leftover::*,
    withdraw_migration_fee::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MeteoraDbcInstruction {
    ClaimCreatorTradingFee {
        data: ClaimCreatorTradingFee,
        accounts: ClaimCreatorTradingFeeInstructionAccounts,
    },

    ClaimPartnerPoolCreationFee {
        data: ClaimPartnerPoolCreationFee,
        accounts: ClaimPartnerPoolCreationFeeInstructionAccounts,
    },

    ClaimProtocolFee {
        data: ClaimProtocolFee,
        accounts: ClaimProtocolFeeInstructionAccounts,
    },

    ClaimProtocolPoolCreationFee {
        data: ClaimProtocolPoolCreationFee,
        accounts: ClaimProtocolPoolCreationFeeInstructionAccounts,
    },

    ClaimTradingFee {
        data: ClaimTradingFee,
        accounts: ClaimTradingFeeInstructionAccounts,
    },

    CloseClaimProtocolFeeOperator {
        data: CloseClaimProtocolFeeOperator,
        accounts: CloseClaimProtocolFeeOperatorInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateClaimProtocolFeeOperator {
        data: CreateClaimProtocolFeeOperator,
        accounts: CreateClaimProtocolFeeOperatorInstructionAccounts,
    },

    CreateConfig {
        data: CreateConfig,
        accounts: CreateConfigInstructionAccounts,
    },

    CreateLocker {
        data: CreateLocker,
        accounts: CreateLockerInstructionAccounts,
    },

    CreatePartnerMetadata {
        data: CreatePartnerMetadata,
        accounts: CreatePartnerMetadataInstructionAccounts,
    },

    CreateVirtualPoolMetadata {
        data: CreateVirtualPoolMetadata,
        accounts: CreateVirtualPoolMetadataInstructionAccounts,
    },

    CreatorWithdrawSurplus {
        data: CreatorWithdrawSurplus,
        accounts: CreatorWithdrawSurplusInstructionAccounts,
    },

    InitializeVirtualPoolWithSplToken {
        data: InitializeVirtualPoolWithSplToken,
        accounts: InitializeVirtualPoolWithSplTokenInstructionAccounts,
    },

    InitializeVirtualPoolWithToken2022 {
        data: InitializeVirtualPoolWithToken2022,
        accounts: InitializeVirtualPoolWithToken2022InstructionAccounts,
    },

    MigrateMeteoraDamm {
        data: MigrateMeteoraDamm,
        accounts: MigrateMeteoraDammInstructionAccounts,
    },

    MigrateMeteoraDammClaimLpToken {
        data: MigrateMeteoraDammClaimLpToken,
        accounts: MigrateMeteoraDammClaimLpTokenInstructionAccounts,
    },

    MigrateMeteoraDammLockLpToken {
        data: MigrateMeteoraDammLockLpToken,
        accounts: MigrateMeteoraDammLockLpTokenInstructionAccounts,
    },

    MigrationDammV2 {
        data: MigrationDammV2,
        accounts: MigrationDammV2InstructionAccounts,
    },

    MigrationDammV2CreateMetadata {
        data: MigrationDammV2CreateMetadata,
        accounts: MigrationDammV2CreateMetadataInstructionAccounts,
    },

    MigrationMeteoraDammCreateMetadata {
        data: MigrationMeteoraDammCreateMetadata,
        accounts: MigrationMeteoraDammCreateMetadataInstructionAccounts,
    },

    PartnerWithdrawSurplus {
        data: PartnerWithdrawSurplus,
        accounts: PartnerWithdrawSurplusInstructionAccounts,
    },

    Swap {
        data: Swap,
        accounts: SwapInstructionAccounts,
    },

    Swap2 {
        data: Swap2,
        accounts: Swap2InstructionAccounts,
    },

    TransferPoolCreator {
        data: TransferPoolCreator,
        accounts: TransferPoolCreatorInstructionAccounts,
    },

    WithdrawLeftover {
        data: WithdrawLeftover,
        accounts: WithdrawLeftoverInstructionAccounts,
    },

    WithdrawMigrationFee {
        data: WithdrawMigrationFee,
        accounts: WithdrawMigrationFeeInstructionAccounts,
    },
}

impl InstructionDecoder for MeteoraDbcDecoder {
    type InstructionType = MeteoraDbcInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = ClaimCreatorTradingFee::decode(data)
                && let Some(accounts) =
                    ClaimCreatorTradingFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::ClaimCreatorTradingFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPartnerPoolCreationFee::decode(data)
                && let Some(accounts) =
                    ClaimPartnerPoolCreationFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::ClaimPartnerPoolCreationFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimProtocolFee::decode(data)
                && let Some(accounts) = ClaimProtocolFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::ClaimProtocolFee { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimProtocolPoolCreationFee::decode(data)
                && let Some(accounts) =
                    ClaimProtocolPoolCreationFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::ClaimProtocolPoolCreationFee {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = ClaimTradingFee::decode(data)
                && let Some(accounts) = ClaimTradingFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::ClaimTradingFee { data, accounts });
            }
        }

        {
            if let Some(data) = CloseClaimProtocolFeeOperator::decode(data)
                && let Some(accounts) =
                    CloseClaimProtocolFeeOperator::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CloseClaimProtocolFeeOperator {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateClaimProtocolFeeOperator::decode(data)
                && let Some(accounts) =
                    CreateClaimProtocolFeeOperator::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreateClaimProtocolFeeOperator {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = CreateConfig::decode(data)
                && let Some(accounts) = CreateConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreateConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreateLocker::decode(data)
                && let Some(accounts) = CreateLocker::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreateLocker { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePartnerMetadata::decode(data)
                && let Some(accounts) =
                    CreatePartnerMetadata::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreatePartnerMetadata { data, accounts });
            }
        }

        {
            if let Some(data) = CreateVirtualPoolMetadata::decode(data)
                && let Some(accounts) =
                    CreateVirtualPoolMetadata::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreateVirtualPoolMetadata { data, accounts });
            }
        }

        {
            if let Some(data) = CreatorWithdrawSurplus::decode(data)
                && let Some(accounts) =
                    CreatorWithdrawSurplus::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::CreatorWithdrawSurplus { data, accounts });
            }
        }

        {
            if let Some(data) = InitializeVirtualPoolWithSplToken::decode(data)
                && let Some(accounts) =
                    InitializeVirtualPoolWithSplToken::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::InitializeVirtualPoolWithSplToken {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = InitializeVirtualPoolWithToken2022::decode(data)
                && let Some(accounts) =
                    InitializeVirtualPoolWithToken2022::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::InitializeVirtualPoolWithToken2022 {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = MigrateMeteoraDamm::decode(data)
                && let Some(accounts) = MigrateMeteoraDamm::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrateMeteoraDamm { data, accounts });
            }
        }

        {
            if let Some(data) = MigrateMeteoraDammClaimLpToken::decode(data)
                && let Some(accounts) =
                    MigrateMeteoraDammClaimLpToken::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrateMeteoraDammClaimLpToken {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = MigrateMeteoraDammLockLpToken::decode(data)
                && let Some(accounts) =
                    MigrateMeteoraDammLockLpToken::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrateMeteoraDammLockLpToken {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = MigrationDammV2::decode(data)
                && let Some(accounts) = MigrationDammV2::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrationDammV2 { data, accounts });
            }
        }

        {
            if let Some(data) = MigrationDammV2CreateMetadata::decode(data)
                && let Some(accounts) =
                    MigrationDammV2CreateMetadata::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrationDammV2CreateMetadata {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = MigrationMeteoraDammCreateMetadata::decode(data)
                && let Some(accounts) =
                    MigrationMeteoraDammCreateMetadata::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::MigrationMeteoraDammCreateMetadata {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = PartnerWithdrawSurplus::decode(data)
                && let Some(accounts) =
                    PartnerWithdrawSurplus::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::PartnerWithdrawSurplus { data, accounts });
            }
        }

        {
            if let Some(data) = Swap::decode(data)
                && let Some(accounts) = Swap::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::Swap { data, accounts });
            }
        }

        {
            if let Some(data) = Swap2::decode(data)
                && let Some(accounts) = Swap2::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::Swap2 { data, accounts });
            }
        }

        {
            if let Some(data) = TransferPoolCreator::decode(data)
                && let Some(accounts) = TransferPoolCreator::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::TransferPoolCreator { data, accounts });
            }
        }

        {
            if let Some(data) = WithdrawLeftover::decode(data)
                && let Some(accounts) = WithdrawLeftover::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::WithdrawLeftover { data, accounts });
            }
        }

        {
            if let Some(data) = WithdrawMigrationFee::decode(data)
                && let Some(accounts) =
                    WithdrawMigrationFee::arrange_accounts(&instruction.accounts)
            {
                return Some(MeteoraDbcInstruction::WithdrawMigrationFee { data, accounts });
            }
        }

        None
    }
}
