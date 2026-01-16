use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

use crate::types::{AmmCreatorFeeOn, CurveParams, MintParams, VestingParams};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PoolCreateEvent {
    pub pool_state: Address,
    pub creator: Address,
    pub config: Address,
    pub base_mint_param: MintParams,
    pub curve_param: CurveParams,
    pub vesting_param: VestingParams,
    pub amm_fee_on: AmmCreatorFeeOn,
}

impl PoolCreateEvent {
    pub const DISCRIMINATOR: [u8; 8] = [151, 215, 226, 9, 118, 161, 115, 174];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
