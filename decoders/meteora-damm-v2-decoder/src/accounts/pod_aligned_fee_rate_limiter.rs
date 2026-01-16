use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PodAlignedFeeRateLimiter {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub fee_increment_bps: u16,
    pub max_limiter_duration: u32,
    pub max_fee_bps: u32,
    pub reference_amount: u64,
}

impl PodAlignedFeeRateLimiter {
    pub const DISCRIMINATOR: [u8; 8] = [160, 219, 8, 251, 179, 7, 16, 117];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
