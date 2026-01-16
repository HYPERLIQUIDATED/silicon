use solana_address::{Address, address};

pub struct RaydiumCpmmDecoder;

pub const PROGRAM_ID: Address = address!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
