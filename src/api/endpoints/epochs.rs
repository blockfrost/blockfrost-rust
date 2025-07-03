use crate::*;
use blockfrost_openapi::models::{
    epoch_content::EpochContent, epoch_param_content::EpochParamContent,
    epoch_stake_content_inner::EpochStakeContentInner,
    epoch_stake_pool_content_inner::EpochStakePoolContentInner,
};

impl BlockfrostAPI {
    pub async fn epochs_latest(&self) -> BlockfrostResult<EpochContent> {
        self.call_endpoint("/epochs/latest".to_string().as_str())
            .await
    }

    pub async fn epochs_latest_parameters(&self) -> BlockfrostResult<EpochParamContent> {
        self.call_endpoint("/epochs/latest/parameters").await
    }

    pub async fn epochs_by_number(&self, number: i32) -> BlockfrostResult<EpochContent> {
        self.call_endpoint(format!("/epochs/{number}").as_str())
            .await
    }

    pub async fn epochs_parameters(&self, number: i32) -> BlockfrostResult<EpochParamContent> {
        self.call_endpoint(format!("/epochs/{number}/parameters").as_str())
            .await
    }

    pub async fn epochs_next(
        &self, number: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<EpochContent>> {
        self.call_paged_endpoint(format!("/epochs/{number}/next").as_str(), pagination)
            .await
    }

    pub async fn epochs_previous(
        &self, number: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<EpochContent>> {
        self.call_paged_endpoint(format!("/epochs/{number}/previous",).as_str(), pagination)
            .await
    }

    pub async fn epochs_stakes(
        &self, number: i32, pagination: Pagination,
    ) -> BlockfrostResult<Vec<EpochStakeContentInner>> {
        self.call_paged_endpoint(format!("/epochs/{number}/stakes").as_str(), pagination)
            .await
    }

    pub async fn epochs_stakes_by_pool(
        &self, number: i32, pool_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<EpochStakePoolContentInner>> {
        self.call_paged_endpoint(
            format!("/epochs/{number}/stakes/{pool_id}").as_str(),
            pagination,
        )
        .await
    }

    pub async fn epochs_blocks(&self, number: i32) -> BlockfrostResult<Vec<String>> {
        self.call_endpoint(format!("/epochs/{number}/blocks").as_str())
            .await
    }

    pub async fn epochs_blocks_by_pool(
        &self, number: i32, pool_id: &str,
    ) -> BlockfrostResult<Vec<String>> {
        self.call_endpoint(format!("/epochs/{number}/blocks/{pool_id}").as_str())
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
        // not building with `json!` macro because this payload is too big
        let mut json_value = serde_json::Map::new();
        json_value.insert("epoch".to_string(), json!(225));
        json_value.insert("min_fee_a".to_string(), json!(44));
        json_value.insert("min_fee_b".to_string(), json!(155381));
        json_value.insert("max_block_size".to_string(), json!(65536));
        json_value.insert("max_tx_size".to_string(), json!(16384));
        json_value.insert("max_block_header_size".to_string(), json!(1100));
        json_value.insert("key_deposit".to_string(), json!("2000000"));
        json_value.insert("pool_deposit".to_string(), json!("500000000"));
        json_value.insert("e_max".to_string(), json!(18));
        json_value.insert("n_opt".to_string(), json!(150));
        json_value.insert("a0".to_string(), json!(0.3));
        json_value.insert("rho".to_string(), json!(0.003));
        json_value.insert("tau".to_string(), json!(0.2));
        json_value.insert("decentralisation_param".to_string(), json!(0.5));
        json_value.insert("extra_entropy".to_string(), json!(null));
        json_value.insert("protocol_major_ver".to_string(), json!(2));
        json_value.insert("protocol_minor_ver".to_string(), json!(0));
        json_value.insert("min_utxo".to_string(), json!("1000000"));
        json_value.insert("min_pool_cost".to_string(), json!("340000000"));
        json_value.insert(
            "nonce".to_string(),
            json!("1a3be38bcbb7911969283716ad7aa550250226b76a61fc51cc9a9a35d9276d81"),
        );
        json_value.insert("price_mem".to_string(), json!(0.001));
        json_value.insert("price_step".to_string(), json!(0.01));
        json_value.insert("max_tx_ex_mem".to_string(), json!("11000000000"));
        json_value.insert("max_tx_ex_steps".to_string(), json!("11000000000"));
        json_value.insert("max_block_ex_mem".to_string(), json!("110000000000"));
        json_value.insert("max_block_ex_steps".to_string(), json!("110000000000"));
        json_value.insert("max_val_size".to_string(), json!("5000"));
        json_value.insert("collateral_percent".to_string(), json!(15));
        json_value.insert("max_collateral_inputs".to_string(), json!(6));
        json_value.insert("coins_per_utxo_word".to_string(), json!("34482"));
        json_value.insert("cost_models".to_string(), json!(null));
        json_value.insert("coins_per_utxo_size".to_string(), json!("34482"));
        json_value.insert("pvt_motion_no_confidence".to_string(), json!(null));
        json_value.insert("pvt_committee_normal".to_string(), json!(null));
        json_value.insert("pvt_committee_no_confidence".to_string(), json!(null));
        json_value.insert("pvt_hard_fork_initiation".to_string(), json!(null));
        json_value.insert("dvt_motion_no_confidence".to_string(), json!(null));
        json_value.insert("dvt_committee_normal".to_string(), json!(null));
        json_value.insert("dvt_committee_no_confidence".to_string(), json!(null));
        json_value.insert("dvt_update_to_constitution".to_string(), json!(null));
        json_value.insert("dvt_hard_fork_initiation".to_string(), json!(null));
        json_value.insert("dvt_p_p_network_group".to_string(), json!(null));
        json_value.insert("dvt_p_p_economic_group".to_string(), json!(null));
        json_value.insert("dvt_p_p_technical_group".to_string(), json!(null));
        json_value.insert("dvt_p_p_gov_group".to_string(), json!(null));
        json_value.insert("dvt_treasury_withdrawal".to_string(), json!(null));
        json_value.insert("committee_min_size".to_string(), json!(null));
        json_value.insert("committee_max_term_length".to_string(), json!(null));
        json_value.insert("gov_action_lifetime".to_string(), json!(null));
        json_value.insert("gov_action_deposit".to_string(), json!(null));
        json_value.insert("drep_deposit".to_string(), json!(null));
        json_value.insert("drep_activity".to_string(), json!(null));
        json_value.insert("pvtpp_security_group".to_string(), json!(null));
        json_value.insert("pvt_p_p_security_group".to_string(), json!(null));
        json_value.insert("min_fee_ref_script_cost_per_byte".to_string(), json!(null));
        let json_value = serde_json::Value::Object(json_value);

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

        serde_json::from_value::<Vec<EpochContent>>(json_value).unwrap();
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

        serde_json::from_value::<Vec<EpochStakeContentInner>>(json_value).unwrap();
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
