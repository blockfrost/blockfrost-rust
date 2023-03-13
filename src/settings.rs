use std::{fmt, time::Duration};

use crate::{
    CARDANO_MAINNET_NETWORK, CARDANO_PREPROD_NETWORK, CARDANO_PREVIEW_NETWORK,
    CARDANO_TESTNET_NETWORK, IPFS_NETWORK,
};

/// Customizable settings for requests made with [`BlockFrostApi`](crate::BlockFrostApi).
#[derive(Debug, Clone)]
pub struct BlockFrostSettings {
    pub network_address: String,
    pub query_parameters: QueryParameters,
    pub retry_settings: RetrySettings,
}

impl BlockFrostSettings {
    /// Create a customizable [`BlockFrostSettings`].
    ///
    /// Default settings are:
    ///
    /// - Network: [`CARDANO_MAINNET_NETWORK`].
    /// - Query parameters: empty.
    /// - Retry settings: disabled.
    pub fn new() -> Self {
        Self {
            network_address: CARDANO_MAINNET_NETWORK.to_owned(),
            query_parameters: QueryParameters::default(),
            retry_settings: RetrySettings::default(),
        }
    }

    /// Change network to [`CARDANO_MAINNET_NETWORK`].
    pub fn use_mainnet(mut self) -> Self {
        self.network_address = CARDANO_MAINNET_NETWORK.to_owned();
        self
    }

    /// Change network to [`CARDANO_TESTNET_NETWORK`].
    pub fn use_testnet(mut self) -> Self {
        self.network_address = CARDANO_TESTNET_NETWORK.to_owned();
        self
    }

    /// Change network to [`CARDANO_PREPROD_NETWORK`].
    pub fn use_preprod(mut self) -> Self {
        self.network_address = CARDANO_PREPROD_NETWORK.to_owned();
        self
    }

    /// Change network to [`CARDANO_PREVIEW_NETWORK`].
    pub fn use_preview(mut self) -> Self {
        self.network_address = CARDANO_PREVIEW_NETWORK.to_owned();
        self
    }
}

/// Customizable settings for requests made with [`IpfsApi`](crate::IpfsApi).
#[derive(Debug, Clone)]
pub struct IpfsSettings {
    pub network_address: String,
    pub query_parameters: QueryParameters,
    pub retry_settings: RetrySettings,
}

impl IpfsSettings {
    /// Create a customizable [`IpfsSettings`].
    ///
    /// # Default settings:
    ///
    /// - Network: [`IPFS_NETWORK`].
    /// - Query parameters: empty.
    /// - Retry settings: disabled.
    pub fn new() -> Self {
        Self {
            network_address: IPFS_NETWORK.to_owned(),
            query_parameters: QueryParameters::default(),
            retry_settings: RetrySettings::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParameters {
    pub(crate) count: Option<u8>,
    pub(crate) page: Option<u32>,
    pub(crate) order: Option<QueryOrder>,
    pub(crate) from: Option<String>,
    pub(crate) to: Option<String>,
}

impl QueryParameters {
    /// Set the "count" query parameter.
    ///
    /// The amount of items that will be retrieved from each page request.
    ///
    /// Defaults to 1.
    ///
    /// The accepted range is 1..=100, if not in range, will be set to the default.
    pub fn set_count(&mut self, count: u8) -> &mut Self {
        let count = if (1..=100).contains(&count) { count } else { 100 };
        self.count = Some(count);
        self
    }

    /// Set the "page" query parameter.
    ///
    /// The number of the page that will be requested, starts at 1.
    ///
    /// Defaults to the first page.
    ///
    /// The accepted range is 1..=21474836, if not in range, will be set to the default.
    pub fn set_page(&mut self, page: u32) -> &mut Self {
        let page = if (1..=100).contains(&page) { page } else { 1 };
        self.page = Some(page);
        self
    }

    /// Set the "order" query parameter.
    ///
    /// The ordering of items inside of the returned page.
    ///
    /// By default, oldest comes first, newest last.
    ///
    /// Defaults to [`QueryOrder::Ascending`].
    pub fn set_order(&mut self, order: QueryOrder) -> &mut Self {
        self.order = Some(order);
        self
    }

    /// Set the "from" query parameter.
    ///
    /// The block number that delimits the start (inclusive) search for results.
    ///
    /// Has to be lower than or equal to the "to" parameter.
    ///
    /// Can optionally contain the index of the block, concatenated using colon.
    ///
    /// # Examples
    /// - Without index: `"8929261"`
    /// - with index: `"8929261:10"`
    pub fn set_from(&mut self, from: String) -> &mut Self {
        self.from = Some(from);
        self
    }

    /// Set the "to" query parameter.
    ///
    /// The block number that delimits the end (inclusive) search for results.
    ///
    /// Has to be higher than or equal to the "from" parameter.
    ///
    /// Can optionally contain the index of the block, concatenated using colon.
    ///
    /// # Examples
    /// - Without index: `"9999269"`
    /// - with index: `"9999269:10"`
    pub fn set_to(&mut self, to: String) -> &mut Self {
        self.to = Some(to);
        self
    }

    /// Removes the parameter added by the [`QueryParameters::set_count`] function.
    pub fn unset_count(&mut self) -> &mut Self {
        self.count = None;
        self
    }

    /// Removes the parameter added by the [`QueryParameters::set_page`] function.
    pub fn unset_page(&mut self) -> &mut Self {
        self.page = None;
        self
    }

    /// Removes the parameter added by the [`QueryParameters::set_order`] function.
    pub fn unset_order(&mut self) -> &mut Self {
        self.order = None;
        self
    }

    /// Removes the parameter added by the [`QueryParameters::set_from`] function.
    pub fn unset_from(&mut self) -> &mut Self {
        self.from = None;
        self
    }

    /// Removes the parameter added by the [`QueryParameters::set_to`] function.
    pub fn unset_to(&mut self) -> &mut Self {
        self.to = None;
        self
    }
}

/// Uses the default network [`CARDANO_MAINNET_NETWORK`].
impl Default for BlockFrostSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// Uses the default network [`IPFS_NETWORK`].
impl Default for IpfsSettings {
    fn default() -> Self {
        Self::new()
    }
}

/// The ordering of items inside of a page.
///
/// By default, oldest comes first, newest last.
///
/// Defaults to [`QueryOrder::Ascending`].
#[derive(Debug, Clone)]
pub enum QueryOrder {
    Ascending,
    Descending,
}

/// Defaults to [`QueryOrder::Ascending`].
impl Default for QueryOrder {
    fn default() -> Self {
        Self::Ascending
    }
}

/// Shows if order is "asc" or "desc", used in URLs.
impl fmt::Display for QueryOrder {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryOrder::Ascending => write!(formatter, "asc"),
            QueryOrder::Descending => write!(formatter, "desc"),
        }
    }
}

/// Settings for retrying when API rate limit is reached.
///
/// Amount and delay are set to zero by default, you will need to change both to enable retrying.
///
/// Retries are only performed when you reach the rate limits (429 status code is retrieved), the
/// quantity depends on your account plan.
///
/// Check different BlockFrost plans and their limits at <https://blockfrost.io/#pricing>.
///
/// Note: You can disable delay between retries with [`Duration::ZERO`].
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RetrySettings {
    pub amount: u64,
    pub delay: Duration,
}

impl RetrySettings {
    /// Create a new `RetrySettings`, with retry amount and delay.
    pub fn new(amount: u64, delay: Duration) -> Self {
        Self { amount, delay }
    }
}
