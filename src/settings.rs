use std::{collections::HashMap, time::Duration};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct BlockFrostSettings {
    pub base_url: Option<String>,
    pub retry_settings: RetrySettings,
    pub headers: HashMap<String, String>,
}

impl BlockFrostSettings {
    pub fn new() -> Self {
        Self {
            base_url: None,
            retry_settings: RetrySettings::default(),
            headers: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct IpfsSettings {
    pub retry_settings: RetrySettings,
    pub headers: HashMap<String, String>,
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
            retry_settings: RetrySettings::default(),
            headers: HashMap::new(),
        }
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

/// Settings for retrying when API rate limit is reached.
///
/// Amount and delay are set to zero by default, you will need to change both to enable retrying.
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
