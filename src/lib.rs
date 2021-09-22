//! Rust SDK for Blockfrost.io

// Internal testing macros
#[macro_use]
mod macros;

mod api;
mod utils;

/// Utils for loading common settings from config file and environment variables.
pub mod env;
/// Custom errors from this crate.
pub mod error;

pub use api::*;
pub use error::*;

/// Definitions of types returned in requests.
pub mod types {
    pub use crate::api::endpoints::*;
}
pub use types::*;

/// Integer used in other types.
///
/// A signed 128-bit integer can store up to 2^127, or 340282366920938463463374607431768211456.
///
/// This number contains 39 digits, the max supply of ADA is 45 billion (11 digits).
pub type Integer = i128;
/// Float type to be used in other types.
///
/// Currently unused.
pub type Float = f64;

/// The URL of the BlockFrost API for the Cardano mainnet.
pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
/// The URL of the BlockFrost API for the Cardano testnet.
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
