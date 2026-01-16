use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtWithdrawIneligibleReward {
    pub pool: Address,
    pub reward_mint: Address,
    pub amount: u64,
}

impl EvtWithdrawIneligibleReward {
    pub const DISCRIMINATOR: [u8; 8] = [248, 215, 184, 78, 31, 180, 179, 168];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
