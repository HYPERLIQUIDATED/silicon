use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct PodAlignedFeeTimeScheduler {
    pub cliff_fee_numerator: u64,
    pub base_fee_mode: u8,
    pub padding: [u8; 5],
    pub number_of_period: u16,
    pub period_frequency: u64,
    pub reduction_factor: u64,
}

impl PodAlignedFeeTimeScheduler {
    pub const DISCRIMINATOR: [u8; 8] = [239, 132, 138, 213, 67, 154, 130, 70];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
