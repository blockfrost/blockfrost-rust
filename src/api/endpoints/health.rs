use crate::*;
use serde::{Deserialize, Serialize};

impl BlockFrostApi {
    /// Root endpoint, points end users to documentation.
    pub async fn root(&self) -> Result<Root> {
        self.get_from_endpoint("/").await
    }

    /// Backend health status as a boolean.
    ///
    /// Your application should handle when backend is unavailable for the given chain.
    pub async fn health(&self) -> Result<Health> {
        self.get_from_endpoint("/health").await
    }

    /// Current backend time.
    ///
    /// This endpoint provides the current UNIX time. Your application might use this to verify
    /// if the client clock is not out of sync.
    pub async fn health_clock(&self) -> Result<HealthClock> {
        self.get_from_endpoint("/health/clock").await
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub url: String,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub is_healthy: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthClock {
    pub server_time: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_root, Root, r#"
    {
      "url": "https://blockfrost.io/",
      "version": "0.1.0"
    }
    "# }

    test_example! { test_health, Health, r#"
    {
      "is_healthy": true
    }
    "# }

    test_example! { test_health_clock, HealthClock, r#"
    {
      "server_time": 1603400958947
    }
    "# }
}
