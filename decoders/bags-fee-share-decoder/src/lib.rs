use solana_address::{Address, address};

pub struct BagsFeeShareDecoder;

pub const PROGRAM_ID: Address = address!("FEE2tBhCKAt7shrod19QttSVREUYPiyMzoku1mL1gqVK");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
