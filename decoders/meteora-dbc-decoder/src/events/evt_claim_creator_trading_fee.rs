use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimCreatorTradingFee {
    pub pool: Address,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

impl EvtClaimCreatorTradingFee {
    pub const DISCRIMINATOR: [u8; 8] = [154, 228, 215, 202, 133, 155, 214, 138];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
