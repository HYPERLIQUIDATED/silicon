use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CreateConfigEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [Address; 8],
    pub coin_creator_fee_basis_points: u64,
    pub admin_set_coin_creator_authority: Address,
}

impl CreateConfigEvent {
    pub const DISCRIMINATOR: [u8; 8] = [107, 52, 89, 129, 55, 226, 81, 22];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
