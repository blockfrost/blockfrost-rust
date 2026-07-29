use crate::*;
use blockfrost_openapi::models::{
    script::Script, script_cbor::ScriptCbor, script_json::ScriptJson,
    script_redeemers_inner::ScriptRedeemersInner, script_utxos_inner::ScriptUtxosInner,
    scripts_inner::ScriptsInner,
};

impl BlockfrostAPI {
    /// Return the list of scripts.
    pub async fn scripts(&self, pagination: Pagination) -> BlockfrostResult<Vec<ScriptsInner>> {
        self.call_paged_endpoint("/scripts", pagination).await
    }

    /// Return information about a specific script.
    pub async fn scripts_by_id(&self, script_hash: &str) -> BlockfrostResult<Script> {
        self.call_endpoint(format!("/scripts/{script_hash}").as_str())
            .await
    }

    /// Return the JSON representation of a timelock script.
    pub async fn scripts_hash_json(&self, script_hash: &str) -> BlockfrostResult<ScriptJson> {
        self.call_endpoint(format!("/scripts/{script_hash}/json").as_str())
            .await
    }

    /// Return the CBOR representation of a Plutus script.
    pub async fn scripts_hash_cbor(&self, script_hash: &str) -> BlockfrostResult<ScriptCbor> {
        self.call_endpoint(format!("/scripts/{script_hash}/cbor").as_str())
            .await
    }

    /// Return the list of redeemers for a specific script.
    pub async fn scripts_redeemers(
        &self, script_hash: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ScriptRedeemersInner>> {
        self.call_paged_endpoint(
            format!("/scripts/{script_hash}/redeemers").as_str(),
            pagination,
        )
        .await
    }

    /// Return the list of UTXOs holding the reference script.
    pub async fn scripts_utxos(
        &self, script_hash: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<ScriptUtxosInner>> {
        self.call_paged_endpoint(format!("/scripts/{script_hash}/utxos").as_str(), pagination)
            .await
    }

    /// Return the JSON value of a datum by its hash.
    pub async fn scripts_datum_hash(
        &self, datum_hash: &str,
    ) -> BlockfrostResult<serde_json::Value> {
        self.call_endpoint(format!("/scripts/datum/{datum_hash}").as_str())
            .await
    }

    /// Return the CBOR representation of a datum by its hash.
    pub async fn scripts_datum_hash_cbor(
        &self, datum_hash: &str,
    ) -> BlockfrostResult<serde_json::Value> {
        self.call_endpoint(format!("/scripts/datum/{datum_hash}/cbor").as_str())
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
    async fn test_scripts_utxos() {
        let json_value = json!([
            {
                "address": "addr1w8qmxkacjdffxah0l3qg8hq2pmvs58q8lcy42zy9kda2ylc6dy5r4",
                "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dce628516157f0",
                "output_index": 0,
                "amount": [
                    {
                        "unit": "lovelace",
                        "quantity": "42000000"
                    },
                    {
                        "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
                        "quantity": "12"
                    }
                ],
                "block": "7eb8e27d18686c7db9a18f8bbcfe34e3fed6e047afaa2d969904d15e934847e6",
                "data_hash": "9e478573ab81ea7a8e31891ce0648b81229f408d596a3483e6f4f9b92d3cf710",
                "inline_datum": null,
                "reference_script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1"
            },
            {
                // inline datum instead of a datum hash
                "address": "addr1w8qmxkacjdffxah0l3qg8hq2pmvs58q8lcy42zy9kda2ylc6dy5r4",
                "tx_hash": "39a7a284c2a0948189dc45dec670211cd4d72f7b66c5726c08d9b3df11e44d58",
                "output_index": 2,
                "amount": [
                    {
                        "unit": "lovelace",
                        "quantity": "729235000"
                    }
                ],
                "block": "953f1b80eb7c11a7ffcd67cbd4fde66e824a451aca5a4065725e5174b81685b7",
                "data_hash": null,
                "inline_datum": "19a6aa",
                "reference_script_hash": "13a3efd825703a352a8f71f4e2758d08c28c564e8dfcce9f77776ad1"
            }
        ]);
        serde_json::from_value::<Vec<ScriptUtxosInner>>(json_value).unwrap();
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
