use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeShareAuthorityHeader {
    pub config: Address,
    pub total_lifetime_accumulated_fees: u128,
    pub total_user_claimed_fees: u128,
    pub total_partner_claimed_fees: u128,
    pub bump: u8,
    pub _padding: [u8; 15],
}

impl FeeShareAuthorityHeader {
    pub const DISCRIMINATOR: [u8; 8] = [82, 28, 140, 167, 168, 216, 93, 211];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
