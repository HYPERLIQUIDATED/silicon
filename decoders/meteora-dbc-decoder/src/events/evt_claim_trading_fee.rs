use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimTradingFee {
    pub pool: Address,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

impl EvtClaimTradingFee {
    pub const DISCRIMINATOR: [u8; 8] = [26, 83, 117, 240, 92, 202, 112, 254];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
