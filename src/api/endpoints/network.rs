use crate::*;
use blockfrost_openapi::models::network::Network;

impl BlockFrostApi {
    pub async fn network(&self) -> Result<Network> {
        self.call_endpoint("/network").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_network, Network, r#"
    {
      "supply": {
        "max": "45000000000000000",
        "total": "32890715183299160",
        "circulating": "32412601976210393",
        "locked": "125006953355"
      },
      "stake": {
        "live": "23204950463991654",
        "active": "22210233523456321"
      }
    }
    "# }
}
