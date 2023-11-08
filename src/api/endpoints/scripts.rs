use crate::*;
use blockfrost_openapi::models::{
    script::Script, script_redeemers_inner::ScriptRedeemersInner, scripts_inner::ScriptsInner,
};

impl BlockfrostAPI {
    pub async fn scripts(&self, pagination: Pagination) -> BlockfrostResult<Vec<ScriptsInner>> {
        self.call_paged_endpoint("/scripts", pagination).await
    }

    pub async fn scripts_by_id(&self, script_hash: &str) -> BlockfrostResult<Script> {
        self.call_endpoint(format!("/scripts/{}", script_hash).as_str())
            .await
    }

    pub async fn scripts_redeemers(
        &self,
        script_hash: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<ScriptRedeemersInner>> {
        self.call_paged_endpoint(
            format!("/scripts/{}/redeemers", script_hash).as_str(),
            pagination,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_scripts() {
        let json_value = json!([
            {
                "script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1"
            },
            {
                "script_hash": "e1457a0c47dfb7a2f6b8fbb059bdceab163c05d34f195b87b9f2b30e"
            },
            {
                "script_hash": "a6e63c0ff05c96943d1cc30bf53112ffff0f34b45986021ca058ec54"
            }
        ]);
        serde_json::from_value::<Vec<ScriptsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_scripts_by_id() {
        let json_value = json!({
            "script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1",
            "type": "plutusV1",
            "serialised_size": 3119
        });
        serde_json::from_value::<Script>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_scripts_redeemers() {
        let json_value = json!([
            {
                "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                "tx_index": 0,
                "redeemer_data_hash": "2a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                "purpose": "spend",
                "datum_hash": "3a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                "unit_mem": "1700",
                "unit_steps": "476468",
                "fee": "172033"
            }
        ]);
        serde_json::from_value::<Vec<ScriptRedeemersInner>>(json_value).unwrap();
    }
}
