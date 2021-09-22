//! Rust SDK for Blockfrost.io

// Internal testing macros
#[macro_use]
mod macros;

mod api;
mod utils;

// Public exports
pub mod env;
pub mod error;
pub use api::*;
pub use error::*;

pub mod types {
    pub use crate::api::endpoints::*;
}
pub use types::*;

/// Integer used in other types.
///
/// A 128-bit integer can store more Lovelace than it currently exists, quadrillion times.
pub type Integer = i128;
/// Float used in other types.
pub type Float = f64;

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
