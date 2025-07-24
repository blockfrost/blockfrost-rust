#![doc(
    html_logo_url = "https://raw.githubusercontent.com/blockfrost/blockfrost-rust/master/docs-logo.svg"
)]
#![doc = include_str!("../README.md")]
mod api;
mod ipfs;
mod pagination;
mod request;
mod settings;
mod url;
mod utils;

pub mod error;
pub mod types;
pub use api::*;
pub use error::*;
pub use ipfs::BlockfrostIPFS;
pub use pagination::Order;
pub use pagination::Pagination;
pub use settings::*;
pub use types::*;

pub const CARDANO_MAINNET_URL: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_PREPROD_URL: &str = "https://cardano-preprod.blockfrost.io/api/v0";
pub const CARDANO_PREVIEW_URL: &str = "https://cardano-preview.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_URL: &str = "https://cardano-testnet.blockfrost.io/api/v0";
pub const IPFS_URL: &str = "https://ipfs.blockfrost.io/api/v0";

pub const DEFAULT_API_VERSION: u32 = 0;
pub const DEFAULT_BATCH_SIZE: u32 = 10;
pub const DEFAULT_ORDER: Order = Order::Asc;
pub const DEFAULT_PAGINATION_PAGE_COUNT: usize = 1;
pub const DEFAULT_PAGINATION_PAGE_ITEMS_COUNT: usize = 100;

/// SDK version being used.
///
/// This is sent on every request as a header.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));
