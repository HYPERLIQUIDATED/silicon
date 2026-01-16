use solana_address::{Address, address};

pub struct MeteoraDbcDecoder;

pub const PROGRAM_ID: Address = address!("dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
