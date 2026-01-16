use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct FeeConfigSnapshotEvent {
    pub timestamp: i64,
    pub admin: Address,
    pub payer: Address,
    pub fee_config: Address,
    pub fee_authority: Address,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub partner: Option<Address>,
    pub partner_bps: u16,
    pub bps: Vec<u16>,
    pub claimers: Vec<Address>,
}

impl FeeConfigSnapshotEvent {
    pub const DISCRIMINATOR: [u8; 8] = [121, 73, 0, 217, 175, 252, 147, 193];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
