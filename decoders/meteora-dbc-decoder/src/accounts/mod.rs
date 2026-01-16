pub mod claim_fee_operator;
pub mod config;
pub mod lock_escrow;
pub mod meteora_damm_migration_metadata;
pub mod partner_metadata;
pub mod pool_config;
pub mod virtual_pool;
pub mod virtual_pool_metadata;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{MeteoraDbcDecoder, PROGRAM_ID};

pub use self::{
    claim_fee_operator::*, config::*, lock_escrow::*, meteora_damm_migration_metadata::*,
    partner_metadata::*, pool_config::*, virtual_pool::*, virtual_pool_metadata::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MeteoraDbcAccount {
    ClaimFeeOperator(Box<ClaimFeeOperator>),
    Config(Box<Config>),
    LockEscrow(Box<LockEscrow>),
    MeteoraDammMigrationMetadata(Box<MeteoraDammMigrationMetadata>),
    PartnerMetadata(Box<PartnerMetadata>),
    PoolConfig(Box<PoolConfig>),
    VirtualPool(Box<VirtualPool>),
    VirtualPoolMetadata(Box<VirtualPoolMetadata>),
}

impl AccountDecoder for MeteoraDbcDecoder {
    type AccountType = MeteoraDbcAccount;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = ClaimFeeOperator::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::ClaimFeeOperator(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Config::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::Config(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = LockEscrow::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::LockEscrow(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = MeteoraDammMigrationMetadata::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::MeteoraDammMigrationMetadata(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PartnerMetadata::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::PartnerMetadata(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PoolConfig::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::PoolConfig(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = VirtualPool::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::VirtualPool(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = VirtualPoolMetadata::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDbcAccount::VirtualPoolMetadata(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
