pub mod claim_a;
pub mod claim_b;
pub mod claim_damm_a;
pub mod claim_damm_b;
pub mod cpi_event;
pub mod create_fee_vaults;
pub mod tweak_fee_vaults;

use silicon_core::{deserialize::ArrangeAccounts, instruction::InstructionDecoder};
use solana_instruction::Instruction;

use crate::{BagsMeteoraFeeClaimerDecoder, PROGRAM_ID};

pub use self::{
    claim_a::*, claim_b::*, claim_damm_a::*, claim_damm_b::*, cpi_event::*, create_fee_vaults::*,
    tweak_fee_vaults::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BagsMeteoraFeeClaimerInstruction {
    ClaimA {
        data: ClaimA,
        accounts: ClaimAInstructionAccounts,
    },

    ClaimB {
        data: ClaimB,
        accounts: ClaimBInstructionAccounts,
    },

    ClaimDammA {
        data: ClaimDammA,
        accounts: ClaimDammAInstructionAccounts,
    },

    ClaimDammB {
        data: ClaimDammB,
        accounts: ClaimDammBInstructionAccounts,
    },

    CpiEvent {
        data: CpiEvent,
        accounts: CpiEventInstructionAccounts,
    },

    CreateFeeVaults {
        data: CreateFeeVaults,
        accounts: CreateFeeVaultsInstructionAccounts,
    },

    TweakFeeVaults {
        data: TweakFeeVaults,
        accounts: TweakFeeVaultsInstructionAccounts,
    },
}

impl InstructionDecoder for BagsMeteoraFeeClaimerDecoder {
    type InstructionType = BagsMeteoraFeeClaimerInstruction;

    fn decode(&self, instruction: &Instruction) -> Option<Self::InstructionType> {
        if instruction.program_id != PROGRAM_ID {
            return None;
        }

        let data = instruction.data.as_slice();

        {
            if let Some(data) = ClaimA::decode(data)
                && let Some(accounts) = ClaimA::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::ClaimA { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimB::decode(data)
                && let Some(accounts) = ClaimB::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::ClaimB { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimDammA::decode(data)
                && let Some(accounts) = ClaimDammA::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::ClaimDammA { data, accounts });
            }
        }

        {
            if let Some(data) = ClaimDammB::decode(data)
                && let Some(accounts) = ClaimDammB::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::ClaimDammB { data, accounts });
            }
        }

        {
            if let Some(data) = CpiEvent::decode(data)
                && let Some(accounts) = CpiEvent::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::CpiEvent { data, accounts });
            }
        }

        {
            if let Some(data) = CreateFeeVaults::decode(data)
                && let Some(accounts) = CreateFeeVaults::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::CreateFeeVaults { data, accounts });
            }
        }

        {
            if let Some(data) = TweakFeeVaults::decode(data)
                && let Some(accounts) = TweakFeeVaults::arrange_accounts(&instruction.accounts)
            {
                return Some(BagsMeteoraFeeClaimerInstruction::TweakFeeVaults { data, accounts });
            }
        }

        None
    }
}
