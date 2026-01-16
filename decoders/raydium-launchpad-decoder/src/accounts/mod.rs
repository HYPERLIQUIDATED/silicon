pub mod global_config;
pub mod platform_config;
pub mod pool_state;
pub mod vesting_record;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{PROGRAM_ID, RaydiumLaunchpadDecoder};

pub use self::{global_config::*, platform_config::*, pool_state::*, vesting_record::*};

#[derive(Debug, Clone, PartialEq)]
pub enum RaydiumLaunchpadAccount {
    GlobalConfig(Box<GlobalConfig>),
    PlatformConfig(Box<PlatformConfig>),
    PoolState(Box<PoolState>),
    VestingRecord(Box<VestingRecord>),
}

impl AccountDecoder for RaydiumLaunchpadDecoder {
    type AccountType = RaydiumLaunchpadAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = GlobalConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumLaunchpadAccount::GlobalConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PlatformConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumLaunchpadAccount::PlatformConfig(Box::new(decoded)),
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
                    data: RaydiumLaunchpadAccount::PoolState(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = VestingRecord::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: RaydiumLaunchpadAccount::VestingRecord(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
