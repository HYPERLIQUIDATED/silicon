use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct ReservedFeeRecipientsEvent {
    pub timestamp: i64,
    pub reserved_fee_recipient: Address,
    pub reserved_fee_recipients: [Address; 7],
}

impl ReservedFeeRecipientsEvent {
    pub const DISCRIMINATOR: [u8; 8] = [43, 188, 250, 18, 221, 75, 187, 95];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
