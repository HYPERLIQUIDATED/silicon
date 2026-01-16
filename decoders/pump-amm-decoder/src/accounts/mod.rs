pub mod bonding_curve;
pub mod fee_config;
pub mod global_config;
pub mod global_volume_accumulator;
pub mod pool;
pub mod sharing_config;
pub mod user_volume_accumulator;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{PROGRAM_ID, PumpAmmDecoder};

pub use self::{
    bonding_curve::*, fee_config::*, global_config::*, global_volume_accumulator::*, pool::*,
    sharing_config::*, user_volume_accumulator::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PumpAmmAccount {
    BondingCurve(Box<BondingCurve>),
    FeeConfig(Box<FeeConfig>),
    GlobalConfig(Box<GlobalConfig>),
    GlobalVolumeAccumulator(Box<GlobalVolumeAccumulator>),
    Pool(Box<Pool>),
    SharingConfig(Box<SharingConfig>),
    UserVolumeAccumulator(Box<UserVolumeAccumulator>),
}

impl AccountDecoder for PumpAmmDecoder {
    type AccountType = PumpAmmAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = BondingCurve::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::BondingCurve(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = FeeConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::FeeConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = GlobalConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::GlobalConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = GlobalVolumeAccumulator::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::GlobalVolumeAccumulator(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Pool::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::Pool(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = SharingConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::SharingConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = UserVolumeAccumulator::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpAmmAccount::UserVolumeAccumulator(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
