use crate::*;
use blockfrost_openapi::models::genesis_content::GenesisContent;

impl BlockFrostApi {
    pub async fn genesis(&self) -> Result<GenesisContent> {
        self.call_endpoint("/genesis").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_genesis() {
        let json_value = json!({
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
        });

        serde_json::from_value::<GenesisContent>(json_value).unwrap();
    }
}
