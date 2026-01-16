use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct EvtUpdateRewardFunder {
    pub pool: Address,
    pub reward_index: u8,
    pub old_funder: Address,
    pub new_funder: Address,
}

impl EvtUpdateRewardFunder {
    pub const DISCRIMINATOR: [u8; 8] = [76, 154, 208, 13, 40, 115, 246, 146];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
