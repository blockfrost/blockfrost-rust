use crate::*;
use blockfrost_openapi::models::{
    epoch_content::EpochContent, epoch_param_content::EpochParamContent,
    epoch_stake_content_inner::EpochStakeContentInner,
    epoch_stake_pool_content_inner::EpochStakePoolContentInner,
};

impl BlockFrostApi {
    pub async fn epochs_latest(&self) -> Result<EpochContent> {
        self.call_endpoint("/epochs/latest".to_string().as_str())
            .await
    }

    pub async fn epochs_latest_parameters(&self) -> Result<EpochParamContent> {
        self.call_endpoint("/epochs/latest/parameters").await
    }

    pub async fn epochs_by_number(&self, number: i32) -> Result<EpochContent> {
        self.call_endpoint(format!("/epochs/{}", number).as_str())
            .await
    }

    pub async fn epochs_parameters(&self, number: i32) -> Result<EpochParamContent> {
        self.call_endpoint(format!("/epochs/{}/parameters", number).as_str())
            .await
    }

    pub async fn epochs_next(
        &self,
        number: i32,
        pagination: Option<Pagination>,
    ) -> Result<Vec<EpochContent>> {
        self.call_paged_endpoint(format!("/epochs/{}/next", number).as_str(), pagination)
            .await
    }

    pub async fn epochs_previous(
        &self,
        number: i32,
        pagination: Option<Pagination>,
    ) -> Result<Vec<EpochContent>> {
        self.call_paged_endpoint(format!("/epochs/{}/previous", number).as_str(), pagination)
            .await
    }

    pub async fn epochs_stakes(
        &self,
        number: i32,
        pagination: Option<Pagination>,
    ) -> Result<Vec<EpochStakeContentInner>> {
        self.call_paged_endpoint(format!("/epochs/{}/stakes", number).as_str(), pagination)
            .await
    }

    pub async fn epochs_stakes_by_pool(
        &self,
        number: i32,
        pool_id: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<EpochStakePoolContentInner>> {
        self.call_paged_endpoint(
            format!("/epochs/{}/stakes/{}", number, pool_id).as_str(),
            pagination,
        )
        .await
    }

    pub async fn epochs_blocks(&self, number: i32) -> Result<Vec<String>> {
        self.call_endpoint(format!("/epochs/{}/blocks", number).as_str())
            .await
    }

    pub async fn epochs_blocks_by_pool(&self, number: i32, pool_id: &str) -> Result<Vec<String>> {
        self.call_endpoint(format!("/epochs/{}/blocks/{}", number, pool_id).as_str())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockfrost_openapi::models::{
        epoch_content::EpochContent, epoch_param_content::EpochParamContent,
    };
    use serde_json::json;

    #[tokio::test]
    async fn test_epochs_latest() {
        let json_value = json!({
            "epoch": 225,
            "start_time": 1603403091,
            "end_time": 1603835086,
            "first_block_time": 1603403092,
            "last_block_time": 1603835084,
            "block_count": 21298,
            "tx_count": 17856,
            "output": "7849943934049314",
            "fees": "4203312194",
            "active_stake": "784953934049314"
        });

        serde_json::from_value::<EpochContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_epochs_latest_parameters() {
        let json_value = json!({
            "epoch": 225,
            "min_fee_a": 44,
            "min_fee_b": 155381,
            "max_block_size": 65536,
            "max_tx_size": 16384,
            "max_block_header_size": 1100,
            "key_deposit": "2000000",
            "pool_deposit": "500000000",
            "e_max": 18,
            "n_opt": 150,
            "a0": 0.3,
            "rho": 0.003,
            "tau": 0.2,
            "decentralisation_param": 0.5,
            "extra_entropy": null,
            "protocol_major_ver": 2,
            "protocol_minor_ver": 0,
            "min_utxo": "1000000",
            "min_pool_cost": "340000000",
            "nonce": "1a3be38bcbb7911969283716ad7aa550250226b76a61fc51cc9a9a35d9276d81",
            "price_mem": 0.001,
            "price_step": 0.01,
            "max_tx_ex_mem": "11000000000",
            "max_tx_ex_steps": "11000000000",
            "max_block_ex_mem": "110000000000",
            "max_block_ex_steps": "110000000000",
            "max_val_size": "5000",
            "collateral_percent": 15,
            "max_collateral_inputs": 6,
            "coins_per_utxo_word": "34482"
        });

        serde_json::from_value::<EpochParamContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_epochs_next() {
        let json_value = json!([
            {
                "epoch": 225,
                "start_time": 1603403091,
                "end_time": 1603835086,
                "first_block_time": 1603403092,
                "last_block_time": 1603835084,
                "block_count": 21298,
                "tx_count": 17856,
                "output": "7849943934049314",
                "fees": "4203312194",
                "active_stake": "784953934049314"
            }
        ]);

        serde_json::from_value::<EpochContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_epochs_stakes() {
        let json_value = json!([
            {
                "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "amount": "4440295078"
            }
        ]);

        serde_json::from_value::<EpochStakeContentInner>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_epochs_stakes_by_pool() {
        let json_value = json!([
            {
                "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
                "amount": "4440295078"
            }
        ]);

        serde_json::from_value::<Vec<EpochStakePoolContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_epochs_blocks() {
        let json_value = json!([
            "d0fa315687e99ccdc96b14cc2ea74a767405d64427b648c470731a9b69e4606e",
            "38bc6efb92a830a0ed22a64f979d120d26483fd3c811f6622a8c62175f530878",
            "f3258fcd8b975c061b4fcdcfcbb438807134d6961ec278c200151274893b6b7d"
        ]);

        serde_json::from_value::<Vec<String>>(json_value).unwrap();
    }
}
