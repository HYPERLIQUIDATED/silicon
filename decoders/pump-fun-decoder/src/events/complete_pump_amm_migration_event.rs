use borsh::{BorshDeserialize, BorshSerialize};
use solana_address::Address;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct CompletePumpAmmMigrationEvent {
    pub user: Address,
    pub mint: Address,
    pub mint_amount: u64,
    pub sol_amount: u64,
    pub pool_migration_fee: u64,
    pub bonding_curve: Address,
    pub timestamp: i64,
    pub pool: Address,
}

impl CompletePumpAmmMigrationEvent {
    pub const DISCRIMINATOR: [u8; 8] = [189, 233, 93, 185, 92, 148, 234, 148];

    #[must_use]
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut data = data.strip_prefix(&Self::DISCRIMINATOR)?;
        Self::deserialize(&mut data).ok()
    }
}
