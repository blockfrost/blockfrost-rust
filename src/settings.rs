use std::time::Duration;

#[derive(Debug, Clone)]
pub struct BlockFrostSettings {
    pub retry_settings: RetrySettings,
}

impl BlockFrostSettings {
    pub fn new() -> Self {
        Self {
            retry_settings: RetrySettings::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IpfsSettings {
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
            retry_settings: RetrySettings::default(),
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
