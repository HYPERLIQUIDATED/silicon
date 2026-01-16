use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub struct MigratedPoolFee {
    pub collect_fee_mode: u8,
    pub dynamic_fee: u8,
    pub pool_fee_bps: u16,
}
