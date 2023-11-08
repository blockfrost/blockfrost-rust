use crate::*;
use blockfrost_openapi::models::{
    __get_200_response::Get200Response, _health_clock_get_200_response::HealthClockGet200Response,
    _health_get_200_response::HealthGet200Response,
};

impl BlockFrostApi {
    /// Root endpoint, points end users to documentation.
    pub async fn root(&self) -> BlockfrostResult<Get200Response> {
        self.call_endpoint("/").await
    }

    /// Backend health status as a boolean.
    pub async fn health(&self) -> BlockfrostResult<HealthGet200Response> {
        self.call_endpoint("/health").await
    }

    /// Current backend time.
    pub async fn health_clock(&self) -> BlockfrostResult<HealthClockGet200Response> {
        self.call_endpoint("/health/clock").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_root() {
        let json_value = json!({
            "url": "https://blockfrost.io/",
            "version": "0.1.0"
        });

        serde_json::from_value::<Get200Response>(json_value).unwrap();
    }

    #[test]
    fn test_health() {
        let json_value = json!({
            "is_healthy": true,
        });

        serde_json::from_value::<HealthGet200Response>(json_value).unwrap();
    }

    #[test]
    fn test_health_clock() {
        let json_value = json!({
            "is_healthy": true,
            "server_time": "1603400958947",
        });

        serde_json::from_value::<HealthGet200Response>(json_value).unwrap();
    }
}
