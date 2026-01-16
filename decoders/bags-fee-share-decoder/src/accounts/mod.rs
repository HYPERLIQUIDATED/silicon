pub mod fee_share_authority_header;
pub mod fee_share_config_header;
pub mod partner_config;
pub mod program_config;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{BagsFeeShareDecoder, PROGRAM_ID};

pub use self::{
    fee_share_authority_header::*, fee_share_config_header::*, partner_config::*, program_config::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum BagsFeeShareAccount {
    FeeShareAuthorityHeader(Box<FeeShareAuthorityHeader>),
    FeeShareConfigHeader(Box<FeeShareConfigHeader>),
    PartnerConfig(Box<PartnerConfig>),
    ProgramConfig(Box<ProgramConfig>),
}

impl AccountDecoder for BagsFeeShareDecoder {
    type AccountType = BagsFeeShareAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = FeeShareAuthorityHeader::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsFeeShareAccount::FeeShareAuthorityHeader(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = FeeShareConfigHeader::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsFeeShareAccount::FeeShareConfigHeader(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PartnerConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsFeeShareAccount::PartnerConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = ProgramConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsFeeShareAccount::ProgramConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
