use crate::*;
use serde::{Deserialize, Serialize};

impl BlockFrostApi {
    pub async fn scripts(&self, pagination: Option<Pagination>) -> Result<Vec<ScriptHash>> {
        self.call_paged_endpoint("/scripts", pagination).await
    }

    pub async fn scripts_by_id(&self, script_hash: &str) -> Result<Script> {
        self.call_endpoint(format!("/scripts/{}", script_hash).as_str())
            .await
    }

    pub async fn scripts_redeemers(
        &self,
        script_hash: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<ScriptRedeemer>> {
        self.call_paged_endpoint(
            format!("/scripts/{}/redeemers", script_hash).as_str(),
            pagination,
        )
        .await
    }
}

/// Created by [`scripts`](BlockFrostApi::scripts) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptHash {
    pub script_hash: String,
}

/// Created by [`scripts_by_id`](BlockFrostApi::scripts_by_id) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    /// Script hash.
    pub script_hash: String,
    /// Type of the script language.
    #[serde(rename = "type")]
    pub type_: ScriptType,
    /// The size of the CBOR serialised script, if a Plutus script.
    pub serialised_size: Option<Integer>,
}

/// Created by [`scripts_redeemers`](BlockFrostApi::scripts_redeemers) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScriptRedeemer {
    /// Hash of the transaction.
    pub tx_hash: String,
    /// The index of the redeemer pointer in the transaction.
    pub tx_index: Integer,
    /// Validation purpose.
    pub purpose: PurposeType,
    /// The budget in Memory to run a script.
    pub unit_mem: String,
    /// The budget in CPU steps to run a script.
    pub unit_steps: String,
    /// The fee consumed to run the script.
    pub fee: String,
}

/// Inner enum for [`Script`].
///
/// The type of the script language.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ScriptType {
    Timelock,
    Plutus,
}

/// Inner enum for [`ScriptRedeemer`].
///
/// Validation purpose.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PurposeType {
    Spend,
    Mint,
    Cert,
    Reward,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_scripts, Vec<ScriptHash>, r#"
    [
      {
        "script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1"
      },
      {
        "script_hash": "e1457a0c47dfb7a2f6b8fbb059bdceab163c05d34f195b87b9f2b30e"
      },
      {
        "script_hash": "a6e63c0ff05c96943d1cc30bf53112ffff0f34b45986021ca058ec54"
      }
    ]
    "# }

    test_example! { test_scripts_by_id, Script, r#"
    {
      "script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1",
      "type": "plutus",
      "serialised_size": 3119
    }
    "# }

    test_example! { test_scripts_redeemers, Vec<ScriptRedeemer>, r#"
    [
      {
        "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
        "tx_index": 0,
        "purpose": "spend",
        "unit_mem": "1700",
        "unit_steps": "476468",
        "fee": "172033"
      }
    ]
    "# }
}
