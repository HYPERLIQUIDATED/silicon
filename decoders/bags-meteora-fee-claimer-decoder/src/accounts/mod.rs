pub mod fee_authority;
pub mod vault;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{BagsMeteoraFeeClaimerDecoder, PROGRAM_ID};

pub use self::{fee_authority::*, vault::*};

#[derive(Debug, Clone, PartialEq)]
pub enum BagsMeteoraFeeClaimerAccount {
    FeeAuthority(Box<FeeAuthority>),
    Vault(Box<Vault>),
}

impl AccountDecoder for BagsMeteoraFeeClaimerDecoder {
    type AccountType = BagsMeteoraFeeClaimerAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = FeeAuthority::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsMeteoraFeeClaimerAccount::FeeAuthority(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Vault::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: BagsMeteoraFeeClaimerAccount::Vault(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
