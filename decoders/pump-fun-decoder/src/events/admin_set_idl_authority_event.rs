use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct AdminSetIdlAuthorityEvent {
    pub idl_authority: Address,
}

impl AdminSetIdlAuthorityEvent {
    pub const DISCRIMINATOR: [u8; 8] = [245, 59, 70, 34, 75, 185, 109, 92];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
