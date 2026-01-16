pub mod config;
pub mod operator;
pub mod pod_aligned_fee_market_cap_scheduler;
pub mod pod_aligned_fee_rate_limiter;
pub mod pod_aligned_fee_time_scheduler;
pub mod pool;
pub mod position;
pub mod token_badge;
pub mod vesting;

use silicon_core::account::{AccountDecoder, DecodedAccount};
use solana_account::Account;

use crate::{MeteoraDammV2Decoder, PROGRAM_ID};

pub use self::{
    config::*, operator::*, pod_aligned_fee_market_cap_scheduler::*,
    pod_aligned_fee_rate_limiter::*, pod_aligned_fee_time_scheduler::*, pool::*, position::*,
    token_badge::*, vesting::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum MeteoraDammV2Account {
    Config(Box<Config>),
    Operator(Box<Operator>),
    PodAlignedFeeMarketCapScheduler(Box<PodAlignedFeeMarketCapScheduler>),
    PodAlignedFeeRateLimiter(Box<PodAlignedFeeRateLimiter>),
    PodAlignedFeeTimeScheduler(Box<PodAlignedFeeTimeScheduler>),
    Pool(Box<Pool>),
    Position(Box<Position>),
    TokenBadge(Box<TokenBadge>),
    Vesting(Box<Vesting>),
}

impl AccountDecoder for MeteoraDammV2Decoder {
    type AccountType = MeteoraDammV2Account;

    fn decode(&self, account: &Account) -> Option<DecodedAccount<Self::AccountType>> {
        if account.owner != PROGRAM_ID {
            return None;
        }

        let data = account.data.as_slice();

        {
            if let Some(decoded) = Config::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::Config(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Operator::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::Operator(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PodAlignedFeeMarketCapScheduler::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::PodAlignedFeeMarketCapScheduler(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PodAlignedFeeRateLimiter::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::PodAlignedFeeRateLimiter(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = PodAlignedFeeTimeScheduler::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::PodAlignedFeeTimeScheduler(Box::new(decoded)),
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
                    data: MeteoraDammV2Account::Pool(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Position::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::Position(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = TokenBadge::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::TokenBadge(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        {
            if let Some(decoded) = Vesting::decode(data) {
                return Some(DecodedAccount {
                    lamports: account.lamports,
                    data: MeteoraDammV2Account::Vesting(Box::new(decoded)),
                    owner: account.owner,
                    executable: account.executable,
                    rent_epoch: account.rent_epoch,
                });
            }
        }

        None
    }
}
