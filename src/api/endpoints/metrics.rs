use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    endpoints! {
        /// Blockfrost usage metrics.
        ///
        /// History of your Blockfrost usage metrics in the past 30 days.
        metrics() -> Vec<Metric> => "/metrics";
            ("https://docs.blockfrost.io/#tag/Metrics/paths/~1metrics~1/get"),

        /// Blockfrost endpoint usage metrics.
        ///
        /// History of your Blockfrost usage metrics per endpoint in the past 30 days.
        metrics_endpoints() -> Vec<MetricEndpoint> => "/metrics/endpoints";
            ("https://docs.blockfrost.io/#tag/Metrics/paths/~1metrics~1endpoints/get"),
    }
}

/// Created by [`metrics`](BlockFrostApi::metrics) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metric {
    /// Starting time of the call count interval (ends midnight UTC) in UNIX time.
    pub time: Integer,
    /// Sum of all calls for a particular day.
    pub calls: Integer,
}

/// Created by [`metrics_endpoints`](BlockFrostApi::metrics_endpoints) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricEndpoint {
    /// Starting time of the call count interval (ends midnight UTC) in UNIX time.
    pub time: u64,
    /// Sum of all calls for a particular day and endpoint.
    pub calls: u64,
    /// Endpoint parent name.
    pub endpoint: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_schema! { test_metric, Vec<Metric>, r#"
    [
      {
        "time": 1612543884,
        "calls": 42
      },
      {
        "time": 1614523884,
        "calls": 6942
      }
    ]
    "# }

    test_schema! { test_metric_endpoint, Vec<MetricEndpoint>, r#"
    [
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
    ]
    "# }
}
