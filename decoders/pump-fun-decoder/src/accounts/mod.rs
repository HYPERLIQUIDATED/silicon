pub mod bonding_curve;
pub mod fee_config;
pub mod global;
pub mod global_volume_accumulator;
pub mod sharing_config;
pub mod user_volume_accumulator;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{PROGRAM_ID, PumpFunDecoder};

pub use self::{
    bonding_curve::*, fee_config::*, global::*, global_volume_accumulator::*, sharing_config::*,
    user_volume_accumulator::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum PumpFunAccount {
    BondingCurve(Box<BondingCurve>),
    FeeConfig(Box<FeeConfig>),
    Global(Box<Global>),
    GlobalVolumeAccumulator(Box<GlobalVolumeAccumulator>),
    SharingConfig(Box<SharingConfig>),
    UserVolumeAccumulator(Box<UserVolumeAccumulator>),
}

impl AccountDecoder for PumpFunDecoder {
    type AccountType = PumpFunAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = BondingCurve::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpFunAccount::BondingCurve(Box::new(decoded)),
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
                    data: PumpFunAccount::FeeConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Global::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: PumpFunAccount::Global(Box::new(decoded)),
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
                    data: PumpFunAccount::GlobalVolumeAccumulator(Box::new(decoded)),
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
                    data: PumpFunAccount::SharingConfig(Box::new(decoded)),
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
                    data: PumpFunAccount::UserVolumeAccumulator(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
