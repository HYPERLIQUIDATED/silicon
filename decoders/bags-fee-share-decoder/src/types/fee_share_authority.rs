use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeShareAuthority {
    pub config: Address,
    pub total_lifetime_accumulated_fees: u128,
    pub total_user_claimed_fees: u128,
    pub total_partner_claimed_fees: u128,
    pub bump: u8,
    pub _padding: [u8; 15],
    pub fees: [u64; 1],
}
