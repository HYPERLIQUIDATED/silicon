use solana_address::{Address, address};

pub struct MeteoraDammV2Decoder;

pub const PROGRAM_ID: Address = address!("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");

pub mod accounts;
pub mod events;
pub mod instructions;
pub mod types;
