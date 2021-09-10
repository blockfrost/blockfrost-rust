use serde::{Deserialize, Serialize};

use crate::*;

impl crate::BlockFrostApi {
    /// Root endpoint.
    ///
    /// Root endpoint has no other function than to point end users to documentation.
    ///
    /// OpenAPI endpoint reference: [`/`].
    ///
    /// [`/`]: https://docs.blockfrost.io/#tag/Health/paths/~1/get
    pub async fn root(&self) -> Result<Root> {
        self.get("/").await
    }

    /// Backend health status.
    ///
    /// Return backend status as a boolean. Your application should handle situations when backend
    /// for the given chain is unavailable.
    ///
    /// OpenAPI endpoint reference: [`/health`].
    ///
    /// [`/health`]: https://docs.blockfrost.io/#tag/Health/paths/~1health/get
    pub async fn health(&self) -> Result<Health> {
        self.get("/health").await
    }

    /// Current backend time.
    ///
    /// This endpoint provides the current UNIX time. Your application might use this to verify
    /// if the client clock is not out of sync.
    ///
    /// OpenAPI endpoint reference: [`/health/clock`].
    ///
    /// [`/health/clock`]: https://docs.blockfrost.io/#tag/Health/paths/~1health~1clock/get
    pub async fn health_clock(&self) -> Result<HealthClock> {
        self.get("/health/clock").await
    }
}

/// Created by [`root`](BlockFrostApi::root) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    /// Points end users to the website url.
    pub url: String,
    /// Current blockfrost backend version.
    pub version: String,
}

/// Created by [`health`](BlockFrostApi::health) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    /// Status of the backend health.
    pub is_healthy: bool,
}

/// Created by [`health_clock`](BlockFrostApi::health_clock) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthClock {
    /// Server UNIX time in milliseconds.
    pub server_time: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_schema! { test_root, Root, r#"
    {
      "url": "https://blockfrost.io/",
      "version": "0.1.0"
    }
    "# }

    test_schema! { test_health, Health, r#"
    {
      "is_healthy": true
    }
    "# }

    test_schema! { test_health_clock, HealthClock, r#"
    {
      "server_time": 1603400958947
    }
    "# }
}
