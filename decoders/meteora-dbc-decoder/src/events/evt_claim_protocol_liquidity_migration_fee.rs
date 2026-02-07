use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtClaimProtocolLiquidityMigrationFee {
    pub pool: Address,
    pub token_base_amount: u64,
    pub token_quote_amount: u64,
}

impl EvtClaimProtocolLiquidityMigrationFee {
    pub const DISCRIMINATOR: [u8; 8] = [81, 168, 116, 31, 161, 86, 27, 35];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
