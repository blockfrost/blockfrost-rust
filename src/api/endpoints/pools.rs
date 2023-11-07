use crate::*;
use blockfrost_openapi::models::{
    pool::Pool, pool_delegators_inner::PoolDelegatorsInner, pool_history_inner::PoolHistoryInner,
    pool_list_retire_inner::PoolListRetireInner, pool_metadata::PoolMetadata,
    pool_updates_inner::PoolUpdatesInner,
    tx_content_pool_certs_inner_relays_inner::TxContentPoolCertsInnerRelaysInner,
};

impl BlockFrostApi {
    pub async fn pools_by_id(&self, pool_id: &str) -> Result<Pool> {
        self.call_endpoint(format!("/pools/{}", pool_id).as_str())
            .await
    }

    pub async fn pools_metadata(&self, pool_id: &str) -> Result<PoolMetadata> {
        self.call_endpoint(format!("/pools/{}/metadata", pool_id).as_str())
            .await
    }

    pub async fn pools(&self, pagination: Option<Pagination>) -> Result<Vec<String>> {
        self.call_paged_endpoint("/pools", pagination).await
    }

    pub async fn pools_retired(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<Vec<PoolListRetireInner>> {
        self.call_paged_endpoint("/pools/retired", pagination).await
    }

    pub async fn pools_retiring(
        &self,
        pagination: Option<Pagination>,
    ) -> Result<Vec<PoolListRetireInner>> {
        self.call_paged_endpoint("/pools/retiring", pagination)
            .await
    }

    pub async fn pools_history(
        &self,
        pool_id: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<PoolHistoryInner>> {
        self.call_paged_endpoint(format!("/pools/{}/history", pool_id).as_str(), pagination)
            .await
    }

    pub async fn pools_relays(
        &self,
        pool_id: &str,
    ) -> Result<Vec<TxContentPoolCertsInnerRelaysInner>> {
        self.call_endpoint(format!("/pools/{}/relays", pool_id).as_str())
            .await
    }

    pub async fn pools_delegators(
        &self,
        pool_id: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<PoolDelegatorsInner>> {
        self.call_paged_endpoint(
            format!("/pools/{}/delegators", pool_id).as_str(),
            pagination,
        )
        .await
    }

    pub async fn pools_blocks(
        &self,
        pool_id: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<String>> {
        self.call_paged_endpoint(format!("/pools/{}/blocks", pool_id).as_str(), pagination)
            .await
    }

    pub async fn pools_updates(
        &self,
        pool_id: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<PoolUpdatesInner>> {
        self.call_paged_endpoint(format!("/pools/{}/updates", pool_id).as_str(), pagination)
            .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_pools() {
        let json_value = json!([
            "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
            "pool1hn7hlwrschqykupwwrtdfkvt2u4uaxvsgxyh6z63703p2knj288",
            "pool1ztjyjfsh432eqetadf82uwuxklh28xc85zcphpwq6mmezavzad2"
        ]);

        serde_json::from_value::<Vec<String>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_retired() {
        let json_value = json!([
            {
                "pool_id": "pool19u64770wqp6s95gkajc8udheske5e6ljmpq33awxk326zjaza0q",
                "epoch": 225
            },
            {
                "pool_id": "pool1dvla4zq98hpvacv20snndupjrqhuc79zl6gjap565nku6et5zdx",
                "epoch": 215
            },
            {
                "pool_id": "pool1wvccajt4eugjtf3k0ja3exjqdj7t8egsujwhcw4tzj4rzsxzw5w",
                "epoch": 231
            }
        ]);

        serde_json::from_value::<Vec<PoolListRetireInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_retiring() {
        let json_value = json!([
            {
                "pool_id": "pool19u64770wqp6s95gkajc8udheske5e6ljmpq33awxk326zjaza0q",
                "epoch": 225
            },
            {
                "pool_id": "pool1dvla4zq98hpvacv20snndupjrqhuc79zl6gjap565nku6et5zdx",
                "epoch": 215
            },
            {
                "pool_id": "pool1wvccajt4eugjtf3k0ja3exjqdj7t8egsujwhcw4tzj4rzsxzw5w",
                "epoch": 231
            }
        ]);
        serde_json::from_value::<Vec<PoolListRetireInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_by_id() {
        let json_value = json!({
          "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
          "hex": "0f292fcaa02b8b2f9b3c8f9fd8e0bb21abedb692a6d5058df3ef2735",
          "vrf_key": "0b5245f9934ec2151116fb8ec00f35fd00e0aa3b075c4ed12cce440f999d8233",
          "blocks_minted": 69,
          "live_stake": "6900000000",
          "live_size": 0.42,
          "live_saturation": 0.93,
          "live_delegators": 127,
          "active_stake": "4200000000",
          "active_size": 0.43,
          "declared_pledge": "5000000000",
          "live_pledge": "5000000001",
          "margin_cost": 0.05,
          "fixed_cost": "340000000",
          "reward_account": "stake1uxkptsa4lkr55jleztw43t37vgdn88l6ghclfwuxld2eykgpgvg3f",
          "owners": [
            "stake1u98nnlkvkk23vtvf9273uq7cph5ww6u2yq2389psuqet90sv4xv9v"
          ],
          "registration": [
            "9f83e5484f543e05b52e99988272a31da373f3aab4c064c76db96643a355d9dc",
            "7ce3b8c433bf401a190d58c8c483d8e3564dfd29ae8633c8b1b3e6c814403e95",
            "3e6e1200ce92977c3fe5996bd4d7d7e192bcb7e231bc762f9f240c76766535b9"
          ],
          "retirement": [
            "252f622976d39e646815db75a77289cf16df4ad2b287dd8e3a889ce14c13d1a8"
          ]
        });
        serde_json::from_value::<Pool>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_history() {
        let json_value = json!([
            {
                "epoch": 233,
                "blocks": 22,
                "active_stake": "20485965693569",
                "active_size": 1.2345,
                "delegators_count": 115,
                "rewards": "206936253674159",
                "fees": "1290968354"
            }
        ]);
        serde_json::from_value::<Vec<PoolHistoryInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_metadata() {
        let json_value = json!({
          "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
          "hex": "0f292fcaa02b8b2f9b3c8f9fd8e0bb21abedb692a6d5058df3ef2735",
          "url": "https://stakenuts.com/mainnet.json",
          "hash": "47c0c68cb57f4a5b4a87bad896fc274678e7aea98e200fa14a1cb40c0cab1d8c",
          "ticker": "NUTS",
          "name": "Stake Nuts",
          "description": "The best pool ever",
          "homepage": "https://stakentus.com/"
        });

        serde_json::from_value::<PoolMetadata>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_relays() {
        let json_value = json!([
            {
                "ipv4": "4.4.4.4",
                "ipv6": "https://stakenuts.com/mainnet.json",
                "dns": "relay1.stakenuts.com",
                "dns_srv": "_relays._tcp.relays.stakenuts.com",
                "port": 3001
            }
        ]);

        serde_json::from_value::<Vec<TxContentPoolCertsInnerRelaysInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pool_delegators() {
        let json_value = json!([
            {
                "address": "stake1ux4vspfvwuus9uwyp5p3f0ky7a30jq5j80jxse0fr7pa56sgn8kha",
                "live_stake": "1137959159981411"
            },
            {
                "address": "stake1uylayej7esmarzd4mk4aru37zh9yz0luj3g9fsvgpfaxulq564r5u",
                "live_stake": "16958865648"
            },
            {
                "address": "stake1u8lr2pnrgf8f7vrs9lt79hc3sxm8s2w4rwvgpncks3axx6q93d4ck",
                "live_stake": "18605647"
            }
        ]);

        serde_json::from_value::<Vec<PoolDelegatorsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_blocks() {
        let json_value = json!([
            "d8982ca42cfe76b747cc681d35d671050a9e41e9cfe26573eb214e94fe6ff21d",
            "026436c539e2ce84c7f77ffe669f4e4bbbb3b9c53512e5857dcba8bb0b4e9a8c",
            "bcc8487f419b8c668a18ea2120822a05df6dfe1de1f0fac3feba88cf760f303c",
            "86bf7b4a274e0f8ec9816171667c1b4a0cfc661dc21563f271acea9482b62df7"
        ]);

        serde_json::from_value::<Vec<String>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_pools_updates() {
        let json_value = json!([
            {
                "tx_hash": "6804edf9712d2b619edb6ac86861fe93a730693183a262b165fcc1ba1bc99cad",
                "cert_index": 0,
                "action": "registered"
            },
            {
                "tx_hash": "9c190bc1ac88b2ab0c05a82d7de8b71b67a9316377e865748a89d4426c0d3005",
                "cert_index": 0,
                "action": "deregistered"
            },
            {
                "tx_hash": "e14a75b0eb2625de7055f1f580d70426311b78e0d36dd695a6bdc96c7b3d80e0",
                "cert_index": 1,
                "action": "registered"
            }
        ]);

        serde_json::from_value::<Vec<PoolUpdatesInner>>(json_value).unwrap();
    }
}
