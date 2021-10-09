use serde::{Deserialize, Serialize};

use crate::*;

impl crate::BlockFrostApi {
    endpoints! {
        /// Return detailed network information.
        network() -> Network => "/network";
            ("https://docs.blockfrost.io/#tag/Cardano-Network/paths/~1network/get"),
    }
}

/// Created by [`network`](BlockFrostApi::network) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Network {
    pub supply: Supply,
    pub stake: Stake,
}

/// Inner member of [`Network`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Supply {
    /// Maximum supply in Lovelaces.
    pub max: String,
    /// Current total (max supply - reserves) supply in Lovelaces.
    pub total: String,
    /// Current circulating (UTXOs + withdrawables) supply in Lovelaces.
    pub circulating: String,
    /// Curent locked supply by scripts in Lovelaces.
    pub locked: String,
}

/// Inner member of [`Network`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stake {
    /// Current live stake in Lovelaces.
    pub live: String,
    /// Current active stake in Lovelaces.
    pub active: String,
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
