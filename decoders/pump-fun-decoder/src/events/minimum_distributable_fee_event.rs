use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MinimumDistributableFeeEvent {
    pub minimum_required: u64,
    pub distributable_fees: u64,
    pub can_distribute: bool,
}

impl MinimumDistributableFeeEvent {
    pub const DISCRIMINATOR: [u8; 8] = [168, 216, 132, 239, 235, 182, 49, 52];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
