use crate::*;
use blockfrost_openapi::models::{
    metrics_endpoints_inner::MetricsEndpointsInner, metrics_inner::MetricsInner,
};

impl BlockFrostApi {
    pub async fn metrics(&self, pagination: Pagination) -> BlockfrostResult<Vec<MetricsInner>> {
        self.call_paged_endpoint("/metrics", pagination).await
    }

    pub async fn metrics_endpoints(
        &self,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<MetricsEndpointsInner>> {
        self.call_paged_endpoint("/metrics/endpoints", pagination)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_metrics() {
        let json_value = json!([
          {
            "time": 1612543884,
            "calls": 42
          },
          {
            "time": 1614523884,
            "calls": 6942
          }
        ]);

        serde_json::from_value::<Vec<MetricsInner>>(json_value).unwrap();
    }

    #[test]
    fn test_metrics_endpoints() {
        let json_value = json!([
          {
            "time": 1612543814,
            "calls": 182,
            "endpoint": "block"
          },
          {
            "time": 1612543814,
            "calls": 42,
            "endpoint": "epoch"
          },
          {
            "time": 1612543812,
            "calls": 775,
            "endpoint": "block"
          },
          {
            "time": 1612523884,
            "calls": 4,
            "endpoint": "epoch"
          },
          {
            "time": 1612553884,
            "calls": 89794,
            "endpoint": "block"
          }
        ]);

        serde_json::from_value::<Vec<MetricsEndpointsInner>>(json_value).unwrap();
    }
}
