//! Rust SDK for Blockfrost.io

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/blockfrost/blockfrost-rust/master/docs-logo.svg"
)]
#![doc = include_str!("../README.md")]

// Internal macros for testing and implementing endpoints
#[macro_use]
mod macros;

mod api;
mod ipfs;
mod settings;
mod url;
mod utils;

pub mod error;
pub mod load;
pub mod types;

pub use api::*;
pub use error::*;
pub use ipfs::*;
pub use lister::*;
pub use settings::*;
pub use types::*;

/// Re-exporting stream functionality from external crates.
///
/// The traits are useful to iterate on streams.
pub mod stream {
    pub use futures::stream::{FuturesOrdered, Stream, StreamExt};
}

/// The URL of the BlockFrost API for the Cardano mainnet.
pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
/// The URL of the BlockFrost API for the Cardano testnet.
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
/// SDK version being used.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));
