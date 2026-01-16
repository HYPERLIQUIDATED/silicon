pub mod claim_damm_v2;
pub mod claim_dbc;
pub mod claim_partner;
pub mod claim_platform_fees;
pub mod claim_user;
pub mod confirm_admin;
pub mod cpi_event;
pub mod create_fee_config;
pub mod create_partner_config;
pub mod dummy_1;
pub mod dummy_2;
pub mod extend_created_fee_config;
pub mod force_claim_user;
pub mod init_program_config;
pub mod update_fee_config;
pub mod update_partner_config;
pub mod update_partner_fee_collection;
pub mod update_program_config;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{BagsFeeShareDecoder, PROGRAM_ID};

pub use self::{
    claim_damm_v2::*, claim_dbc::*, claim_partner::*, claim_platform_fees::*, claim_user::*,
    confirm_admin::*, cpi_event::*, create_fee_config::*, create_partner_config::*, dummy_1::*,
    dummy_2::*, extend_created_fee_config::*, force_claim_user::*, init_program_config::*,
    update_fee_config::*, update_partner_config::*, update_partner_fee_collection::*,
    update_program_config::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BagsFeeShareInstruction {
    ClaimDammV2 {
        data: ClaimDammV2,
        accounts: ClaimDammV2InstructionAccounts,
    },

    ClaimDbc {
        data: ClaimDbc,
        accounts: ClaimDbcInstructionAccounts,
    },

    ClaimPartner {
        data: ClaimPartner,
        accounts: ClaimPartnerInstructionAccounts,
    },

    ClaimPlatformFees {
        data: ClaimPlatformFees,
        accounts: ClaimPlatformFeesInstructionAccounts,
    },

    ClaimUser {
        data: ClaimUser,
        accounts: ClaimUserInstructionAccounts,
    },

    ConfirmAdmin {
        data: ConfirmAdmin,
        accounts: ConfirmAdminInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateFeeConfig {
        data: CreateFeeConfig,
        accounts: CreateFeeConfigInstructionAccounts,
    },

    CreatePartnerConfig {
        data: CreatePartnerConfig,
        accounts: CreatePartnerConfigInstructionAccounts,
    },

    Dummy1 {
        data: Dummy1,
        accounts: Dummy1InstructionAccounts,
    },

    Dummy2 {
        data: Dummy2,
        accounts: Dummy2InstructionAccounts,
    },

    ExtendCreatedFeeConfig {
        data: ExtendCreatedFeeConfig,
        accounts: ExtendCreatedFeeConfigInstructionAccounts,
    },

    ForceClaimUser {
        data: ForceClaimUser,
        accounts: ForceClaimUserInstructionAccounts,
    },

    InitProgramConfig {
        data: InitProgramConfig,
        accounts: InitProgramConfigInstructionAccounts,
    },

    UpdateFeeConfig {
        data: UpdateFeeConfig,
        accounts: UpdateFeeConfigInstructionAccounts,
    },

    UpdatePartnerConfig {
        data: UpdatePartnerConfig,
        accounts: UpdatePartnerConfigInstructionAccounts,
    },

    UpdatePartnerFeeCollection {
        data: UpdatePartnerFeeCollection,
        accounts: UpdatePartnerFeeCollectionInstructionAccounts,
    },

    UpdateProgramConfig {
        data: UpdateProgramConfig,
        accounts: UpdateProgramConfigInstructionAccounts,
    },
}

impl InstructionDecoder for BagsFeeShareDecoder {
    type InstructionType = BagsFeeShareInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = ClaimDammV2::decode(data)
                && let Some(accounts) = ClaimDammV2::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ClaimDammV2 { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimDbc::decode(data)
                && let Some(accounts) = ClaimDbc::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ClaimDbc { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPartner::decode(data)
                && let Some(accounts) = ClaimPartner::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ClaimPartner { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimPlatformFees::decode(data)
                && let Some(accounts) = ClaimPlatformFees::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ClaimPlatformFees { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimUser::decode(data)
                && let Some(accounts) = ClaimUser::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ClaimUser { data, accounts });
            }
        }

        {
            if let Some(data) = ConfirmAdmin::decode(data)
                && let Some(accounts) = ConfirmAdmin::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ConfirmAdmin { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateFeeConfig::decode(data)
                && let Some(accounts) = CreateFeeConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::CreateFeeConfig { data, accounts });
            }
        }

        {
            if let Some(data) = CreatePartnerConfig::decode(data)
                && let Some(accounts) = CreatePartnerConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::CreatePartnerConfig { data, accounts });
            }
        }

        {
            if let Some(data) = Dummy1::decode(data)
                && let Some(accounts) = Dummy1::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::Dummy1 { data, accounts });
            }
        }

        {
            if let Some(data) = Dummy2::decode(data)
                && let Some(accounts) = Dummy2::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::Dummy2 { data, accounts });
            }
        }

        {
            if let Some(data) = ExtendCreatedFeeConfig::decode(data)
                && let Some(accounts) =
                    ExtendCreatedFeeConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ExtendCreatedFeeConfig { data, accounts });
            }
        }

        {
            if let Some(data) = ForceClaimUser::decode(data)
                && let Some(accounts) = ForceClaimUser::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::ForceClaimUser { data, accounts });
            }
        }

        {
            if let Some(data) = InitProgramConfig::decode(data)
                && let Some(accounts) = InitProgramConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::InitProgramConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdateFeeConfig::decode(data)
                && let Some(accounts) = UpdateFeeConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::UpdateFeeConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePartnerConfig::decode(data)
                && let Some(accounts) = UpdatePartnerConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::UpdatePartnerConfig { data, accounts });
            }
        }

        {
            if let Some(data) = UpdatePartnerFeeCollection::decode(data)
                && let Some(accounts) =
                    UpdatePartnerFeeCollection::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::UpdatePartnerFeeCollection {
                    data,
                    accounts,
                });
            }
        }

        {
            if let Some(data) = UpdateProgramConfig::decode(data)
                && let Some(accounts) = UpdateProgramConfig::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsFeeShareInstruction::UpdateProgramConfig { data, accounts });
            }
        }

        None
    }
}
