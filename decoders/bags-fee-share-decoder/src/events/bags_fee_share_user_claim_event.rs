use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct BagsFeeShareUserClaimEvent {
    pub timestamp: i64,
    pub base_mint: Address,
    pub quote_mint: Address,
    pub user: Address,
    pub fee_share_config: Address,
    pub fee_share_authority: Address,
    pub claimer_index: u32,
    pub claimed: u64,
    pub is_forced: bool,
}

impl BagsFeeShareUserClaimEvent {
    pub const DISCRIMINATOR: [u8; 8] = [115, 178, 50, 219, 175, 234, 48, 101];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
