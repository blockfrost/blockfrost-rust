#![doc(
    html_logo_url = "https://raw.githubusercontent.com/blockfrost/blockfrost-rust/master/docs-logo.svg"
)]
#![doc = include_str!("../README.md")]
mod api;
mod ipfs;
mod request;
mod settings;
mod url;
mod utils;

pub mod error;
pub mod load;
pub mod types;
pub use api::*;
pub use error::*;
pub use ipfs::IpfsApi;
pub use settings::*;
pub use types::*;

pub const CARDANO_MAINNET: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_PREPROD: &str = "https://cardano-preprod.blockfrost.io/api/v0";
pub const CARDANO_PREVIEW: &str = "https://cardano-preview.blockfrost.io/api/v0";
pub const CARDANO_TESTNET: &str = "https://cardano-testnet.blockfrost.io/api/v0";
pub const IPFS: &str = "https://ipfs.blockfrost.io/api/v0";

/// SDK version being used.
///
/// This is sent on every request as a header.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));
