use crate::*;
use blockfrost_openapi::models::{network::Network, network_eras_inner::NetworkErasInner};

impl BlockfrostAPI {
    /// Return detailed network information.
    pub async fn network(&self) -> BlockfrostResult<Network> {
        self.call_endpoint("/network").await
    }

    /// Return the list of network eras.
    pub async fn network_eras(&self) -> BlockfrostResult<Vec<NetworkErasInner>> {
        self.call_endpoint("/network/eras").await
    }
}

#[cfg(test)]
mod tests {
    use blockfrost_openapi::models::network::Network;
    use serde_json::json;

    #[test]
    fn test_network() {
        let json_value = json!({
          "supply": {
            "max": "45000000000000000",
            "total": "32890715183299160",
            "circulating": "32412601976210393",
            "locked": "125006953355",
            "treasury": "98635632000000",
            "reserves": "46635632000000"
          },
          "stake": {
            "live": "23204950463991654",
            "active": "22210233523456321"
          }
        });

        serde_json::from_value::<Network>(json_value).unwrap();
    }
}
