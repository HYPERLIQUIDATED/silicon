use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtCurveComplete {
    pub pool: Address,
    pub config: Address,
    pub base_reserve: u64,
    pub quote_reserve: u64,
}

impl EvtCurveComplete {
    pub const DISCRIMINATOR: [u8; 8] = [229, 231, 86, 84, 156, 134, 75, 24];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
