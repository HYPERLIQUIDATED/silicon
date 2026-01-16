use solana_address::{Address, address};

pub struct PumpAmmDecoder;

pub const PROGRAM_ID: Address = address!("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
