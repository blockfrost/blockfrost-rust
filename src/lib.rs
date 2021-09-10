//! Rust SDK for Blockfrost.io

// Internal testing macros
#[macro_use]
mod macros;

pub mod env;

mod api;
mod error;
mod types;

pub use api::*;
pub use error::*;
pub use types::*;

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
