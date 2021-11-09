use std::{fmt, time::Duration};

use crate::{CARDANO_MAINNET_NETWORK, CARDANO_TESTNET_NETWORK};

/// Configuration of parameters for the requests.
#[derive(Debug, Clone)]
pub struct BlockFrostSettings {
    pub(crate) network_address: String,
    pub(crate) query_parameters: QueryParameters,
}

impl BlockFrostSettings {
    /// Create [`BlockFrostSettings`] with the default settings.
    ///
    /// Defaults are:
    ///
    /// - Network: [`CARDANO_MAINNET_NETWORK`].
    /// - Query parameters: empty.
    pub fn new() -> Self {
        Self {
            network_address: CARDANO_MAINNET_NETWORK.to_string(),
            query_parameters: QueryParameters::default(),
        }
    }

    pub fn use_mainnet(mut self) -> Self {
        self.network_address = CARDANO_MAINNET_NETWORK.to_string();
        self
    }

    pub fn use_testnet(mut self) -> Self {
        self.network_address = CARDANO_TESTNET_NETWORK.to_string();
        self
    }

    pub fn set_network(&mut self, network: impl AsRef<str>) {
        self.network_address = network.as_ref().to_string();
    }

    pub fn current_network(&self) -> &str {
        &self.network_address
    }

    pub fn query_parameters(&self) -> &QueryParameters {
        &self.query_parameters
    }

    pub fn configure<F>(mut self, function: F) -> Self
    where
        F: FnOnce(&mut QueryParameters),
    {
        function(&mut self.query_parameters);
        self
    }
}

#[derive(Debug, Clone)]
pub struct IpfsSettings {
    pub(crate) network_address: String,
    pub(crate) query_parameters: QueryParameters,
}

impl IpfsSettings {
    pub fn new() -> Self {
        Self {
            network_address: "https://ipfs.blockfrost.io/api/v0".to_string(),
            query_parameters: QueryParameters::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueryParameters {
    pub(crate) count: Option<u8>,
    pub(crate) page: Option<u64>,
    pub(crate) order: Option<QueryOrder>,
    pub(crate) from: Option<String>,
    pub(crate) to: Option<String>,
}

impl QueryParameters {
    pub fn set_count(&mut self, count: u8) -> &mut Self {
        self.count = Some(count);
        self
    }

    pub fn set_page(&mut self, page: u64) -> &mut Self {
        self.page = Some(page);
        self
    }

    pub fn set_order(&mut self, order: QueryOrder) -> &mut Self {
        self.order = Some(order);
        self
    }
    pub fn set_from(&mut self, from: String) -> &mut Self {
        self.from = Some(from);
        self
    }

    pub fn set_to(&mut self, to: String) -> &mut Self {
        self.to = Some(to);
        self
    }

    pub fn unset_count(&mut self) -> &mut Self {
        self.count = None;
        self
    }

    pub fn unset_page(&mut self) -> &mut Self {
        self.page = None;
        self
    }

    pub fn unset_order(&mut self) -> &mut Self {
        self.order = None;
        self
    }

    pub fn unset_from(&mut self) -> &mut Self {
        self.from = None;
        self
    }

    pub fn unset_to(&mut self) -> &mut Self {
        self.to = None;
        self
    }
}

impl Default for BlockFrostSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for IpfsSettings {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum QueryOrder {
    Ascending,
    Descending,
}

impl Default for QueryOrder {
    fn default() -> Self {
        Self::Ascending
    }
}

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
/// Retries are only performed when you reach the rate limits (429 status code is retrieved),
/// different plans have different limits.
///
/// Check different plans and their limits at <https://blockfrost.io/#pricing>.
///
/// Note: You can disable delay between retries with [`Duration::ZERO`].
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
