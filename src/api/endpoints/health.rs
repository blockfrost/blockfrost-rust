use crate::*;
use blockfrost_openapi::models::{
    __get_200_response::Get200Response, _health_clock_get_200_response::HealthClockGet200Response,
    _health_get_200_response::HealthGet200Response,
};

impl BlockFrostApi {
    /// Root endpoint, points end users to documentation.
    pub async fn root(&self) -> Result<Get200Response> {
        self.call_endpoint("/").await
    }

    /// Backend health status as a boolean.
    ///
    /// Your application should handle when backend is unavailable for the given chain.
    pub async fn health(&self) -> Result<HealthGet200Response> {
        self.call_endpoint("/health").await
    }

    /// Current backend time.
    ///
    /// This endpoint provides the current UNIX time. Your application might use this to verify
    /// if the client clock is not out of sync.
    pub async fn health_clock(&self) -> Result<HealthClockGet200Response> {
        self.call_endpoint("/health/clock").await
    }
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
