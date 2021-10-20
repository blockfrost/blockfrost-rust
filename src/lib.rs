//! Rust SDK for Blockfrost.io

// Internal testing macros
#[cfg(test)]
#[macro_use]
mod macros;

mod api;
mod ipfs;
mod url;
mod utils;

/// Utils for loading common settings from config file and environment variables.
pub mod env;
/// Custom errors from this crate.
pub mod error;
/// Definitions for types returned in requests.
pub mod types;

pub use api::*;
pub use error::*;
pub use ipfs::*;
pub use lister::*;
pub use types::*;

/// Re-exporting stream functionality from external crates.
///
/// These traits are necessary to work with asynchronous iterators.
pub mod stream {
    pub use futures::stream::{FuturesOrdered, Stream, StreamExt};
}

/// The URL of the BlockFrost API for the Cardano mainnet.
pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
/// The URL of the BlockFrost API for the Cardano testnet.
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
/// SDK version being used.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));
