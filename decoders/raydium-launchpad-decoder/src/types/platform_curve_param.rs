use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::BondingCurveParam;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PlatformCurveParam {
    pub epoch: u64,
    pub index: u8,
    pub global_config: Address,
    pub bonding_curve_param: BondingCurveParam,
    pub padding: [u64; 50],
}
