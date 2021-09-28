use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    endpoints! {
        /// Return the information about blockchain genesis.
        genesis() -> Genesis => "/genesis";
            ("https://docs.blockfrost.io/#tag/Cardano-Ledger/paths/~1genesis/get"),
    }
}

/// Created by [`genesis`](BlockFrostApi::genesis) method.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genesis {
    /// The proportion of slots in which blocks should be issued.
    pub active_slots_coefficient: Float,
    /// Determines the quorum needed for votes on the protocol parameter updates.
    pub update_quorum: Integer,
    /// The total number of lovelace in the system.
    pub max_lovelace_supply: String,
    /// Network identifier.
    pub network_magic: Integer,
    /// Number of slots in an epoch.
    pub epoch_length: Integer,
    /// Time of slot 0 in UNIX time.
    pub system_start: Integer,
    /// Number of slots in an KES period.
    pub slots_per_kes_period: Integer,
    /// Duration of one slot in seconds.
    pub slot_length: Integer,
    /// The maximum number of time a KES key can be evolved before a pool operator must create a new operational certificate.
    pub max_kes_evolutions: Integer,
    /// Security parameter k.
    pub security_param: Integer,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_genesis, Genesis, r#"
    {
      "active_slots_coefficient": 0.05,
      "update_quorum": 5,
      "max_lovelace_supply": "45000000000000000",
      "network_magic": 764824073,
      "epoch_length": 432000,
      "system_start": 1506203091,
      "slots_per_kes_period": 129600,
      "slot_length": 1,
      "max_kes_evolutions": 62,
      "security_param": 2160
    }
    "# }
}
