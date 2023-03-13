#![doc(
    html_logo_url = "https://raw.githubusercontent.com/blockfrost/blockfrost-rust/master/docs-logo.svg"
)]
#![doc = include_str!("../README.md")]

// Internal macros for testing and implementing endpoints
#[macro_use]
mod macros;

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

/// [`Lister`] stream.
///
/// [`Lister`] is a infinite asynchronous iterator that helps with pagination.
///
/// An asynchronous iterator is called a _Stream_. Rust does not support iterating through streams
/// with the `for` loop, to solve that, you might want to use the provided method
/// [`next`](futures::stream::StreamExt::next) from the [`futures::stream::StreamExt`] trait.
///
/// For convenience, those _Stream_ traits from the [`futures`] crate are re-exported in this
/// module.
///
/// # Example (from [`lister.rs`]):
///
/// ```
/// use blockfrost::{BlockFrostApi, stream::StreamExt};
///
/// async fn list_pages() -> blockfrost::Result<()> {
///     let api = BlockFrostApi::new("project_id", Default::default());
///     let block_number = "4874756";
///     let mut block_lister = api.blocks_previous_all(block_number).take(10);
///
///     while let Some(page) = block_lister.next().await {
///         let page = page?;
///         dbg!(page);
///     }
///
///     Ok(())
/// }
/// ```
///
/// [`lister.rs`]: https://github.com/blockfrost/blockfrost-rust/blob/master/examples/lister.rs
/// [`Lister`]: crate::stream::Lister
pub mod stream {
    pub use crate::api::lister::Lister;
    pub use futures::stream::{Stream, StreamExt};
}

/// The URL of the [BlockFrost API](https://docs.blockfrost.io) for the Cardano mainnet.
pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
/// The URL of the [BlockFrost API](https://docs.blockfrost.io) for the Cardano preprod net.
pub const CARDANO_PREPROD_NETWORK: &str = "https://cardano-preprod.blockfrost.io/api/v0";
/// The URL of the [BlockFrost API](https://docs.blockfrost.io) for the Cardano preview net.
pub const CARDANO_PREVIEW_NETWORK: &str = "https://cardano-preview.blockfrost.io/api/v0";
/// The URL of the [BlockFrost API](https://docs.blockfrost.io) for the Cardano testnet.
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";
/// The URL of the BlockFrost IPFS network.
pub const IPFS_NETWORK: &str = "https://ipfs.blockfrost.io/api/v0";

/// SDK version being used.
///
/// This is sent on every request as a header.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));
