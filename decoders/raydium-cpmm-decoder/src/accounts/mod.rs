pub mod amm_config;
pub mod observation_state;
pub mod permission;
pub mod pool_state;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{PROGRAM_ID, RaydiumCpmmDecoder};

pub use self::{amm_config::*, observation_state::*, permission::*, pool_state::*};

#[derive(Debug, Clone, PartialEq)]
pub enum RaydiumCpmmAccount {
    AmmConfig(Box<AmmConfig>),
    ObservationState(Box<ObservationState>),
    Permission(Box<Permission>),
    PoolState(Box<PoolState>),
}

impl AccountDecoder for RaydiumCpmmDecoder {
    type AccountType = RaydiumCpmmAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = AmmConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumCpmmAccount::AmmConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = ObservationState::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumCpmmAccount::ObservationState(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Permission::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumCpmmAccount::Permission(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PoolState::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumCpmmAccount::PoolState(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
