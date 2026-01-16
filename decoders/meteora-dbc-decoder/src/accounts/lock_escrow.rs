use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct LockEscrow {
    pub pool: Address,
    pub owner: Address,
    pub escrow_vault: Address,
    pub bump: u8,
    pub total_locked_amount: u64,
    pub lp_per_token: u128,
    pub unclaimed_fee_pending: u64,
    pub a_fee: u64,
    pub b_fee: u64,
}

impl LockEscrow {
    pub const DISCRIMINATOR: [u8; 8] = [190, 106, 121, 6, 200, 182, 21, 75];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
