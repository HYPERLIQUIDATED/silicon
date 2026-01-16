pub mod claim_protocol;
pub mod claim_user_parameters;
pub mod create_fee_config_parameters;
pub mod create_partner_config_parameters;
pub mod extend_created_fee_config_parameters;
pub mod fee_share_authority;
pub mod fee_share_config;
pub mod force_claim_user_parameters;
pub mod update_fee_config_parameters;
pub mod update_partner_config_parameters;
pub mod update_partner_fee_collection_parameters;
pub mod update_program_config_parameters;

pub use self::{
    claim_protocol::*, claim_user_parameters::*, create_fee_config_parameters::*,
    create_partner_config_parameters::*, extend_created_fee_config_parameters::*,
    fee_share_authority::*, fee_share_config::*, force_claim_user_parameters::*,
    update_fee_config_parameters::*, update_partner_config_parameters::*,
    update_partner_fee_collection_parameters::*, update_program_config_parameters::*,
};
