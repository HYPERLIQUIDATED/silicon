use solana_address::{Address, address};

pub struct RaydiumLaunchpadDecoder;

pub const PROGRAM_ID: Address = address!("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
