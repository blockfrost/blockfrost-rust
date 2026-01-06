use crate::*;
use blockfrost_openapi::models::{
    pool::Pool, pool_delegators_inner::PoolDelegatorsInner, pool_history_inner::PoolHistoryInner,
    pool_list_extended_inner::PoolListExtendedInner, pool_list_retire_inner::PoolListRetireInner,
    pool_metadata::PoolMetadata, pool_updates_inner::PoolUpdatesInner,
    tx_content_pool_certs_inner_relays_inner::TxContentPoolCertsInnerRelaysInner,
};

impl BlockfrostAPI {
    pub async fn pools_by_id(&self, pool_id: &str) -> BlockfrostResult<Pool> {
        self.call_endpoint(format!("/pools/{pool_id}").as_str())
            .await
    }

    pub async fn pools_metadata(&self, pool_id: &str) -> BlockfrostResult<PoolMetadata> {
        self.call_endpoint(format!("/pools/{pool_id}/metadata").as_str())
            .await
    }

    pub async fn pools(&self, pagination: Pagination) -> BlockfrostResult<Vec<String>> {
        self.call_paged_endpoint("/pools", pagination).await
    }

    pub async fn pools_extended(
        &self, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolListExtendedInner>> {
        self.call_paged_endpoint("/pools/extended", pagination)
            .await
    }

    pub async fn pools_retired(
        &self, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolListRetireInner>> {
        self.call_paged_endpoint("/pools/retired", pagination).await
    }

    pub async fn pools_retiring(
        &self, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolListRetireInner>> {
        self.call_paged_endpoint("/pools/retiring", pagination)
            .await
    }

    pub async fn pools_history(
        &self, pool_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolHistoryInner>> {
        self.call_paged_endpoint(format!("/pools/{pool_id}/history").as_str(), pagination)
            .await
    }

    pub async fn pools_relays(
        &self, pool_id: &str,
    ) -> BlockfrostResult<Vec<TxContentPoolCertsInnerRelaysInner>> {
        self.call_endpoint(format!("/pools/{pool_id}/relays").as_str())
            .await
    }

    pub async fn pools_delegators(
        &self, pool_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolDelegatorsInner>> {
        self.call_paged_endpoint(format!("/pools/{pool_id}/delegators").as_str(), pagination)
            .await
    }

    pub async fn pools_blocks(
        &self, pool_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<String>> {
        self.call_paged_endpoint(format!("/pools/{pool_id}/blocks").as_str(), pagination)
            .await
    }

    pub async fn pools_updates(
        &self, pool_id: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<PoolUpdatesInner>> {
        self.call_paged_endpoint(format!("/pools/{pool_id}/updates").as_str(), pagination)
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
    async fn test_pools_extended() {
        let json_value = json!([
            {
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "hex": "0f292fcaa02b8b2f9b3c8f9fd8e0bb21abedb692a6d5058df3ef2735",
                "active_stake": "5727090990610",
                "live_stake": "5721241414066",
                "live_saturation": 0.07458075839782029,
                "blocks_minted": 3512,
                "margin_cost": 0.049,
                "fixed_cost": "340000000",
                "declared_pledge": "250000000000",
                "metadata": {
                    "hash": "47c0c68cb57f4a5b4a87bad896fc274678e7aea98e200fa14a1cb40c0cab1d8c",
                    "url": "https://stakenuts.com/mainnet.json",
                    "ticker": "NUTS",
                    "name": "StakeNuts",
                    "description": "StakeNuts.com",
                    "homepage": "https://stakenuts.com/"
                }
            },
            {
                "pool_id": "pool1ddskftmsscw92d7vnj89pldwx5feegkgcmamgt5t0e4lkd7mdp8",
                "hex": "6b6164af70861c5537cc9c8e50fdae35139ca2c8c6fbb42e8b7e6bfb",
                "active_stake": "2671108363",
                "live_stake": "2671108363",
                "live_saturation": 0.000034819940823598694,
                "blocks_minted": 23,
                "margin_cost": 0.05,
                "fixed_cost": "340000000",
                "declared_pledge": "7149000000",
                "metadata": {
                    "hash": "79e7cf8d936bf0ced040516b288e2edc76f2f87af5400f92010a682de3a052e9",
                    "url": "https://pool.adascan.net/meta/v1/poolmeta.json",
                    "ticker": null,
                    "name": null,
                    "description": null,
                    "homepage": null
                }
            },
            {
                "pool_id": "pool1a5ld0eulnjxwpg6qyjs6hxzqwj0esft7k0qsf6nj8pkfxy5lml3",
                "hex": "ed3ed7e79f9c8ce0a34024a1ab9840749f98257eb3c104ea72386c93",
                "active_stake": "76334298659264",
                "live_stake": "76301000831243",
                "live_saturation": 0.9946419136441588,
                "blocks_minted": 2504,
                "margin_cost": 1,
                "fixed_cost": "340000000",
                "declared_pledge": "73000000000000",
                "metadata": null
            }
        ]);

        serde_json::from_value::<Vec<PoolListExtendedInner>>(json_value).unwrap();
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
            "vrf_key": "b512cc7c1a8ba689c2d8fd27adfdbac2049a3f8f95c8b85e8298f14d7d8dc4e6",
            "blocks_minted": 3512,
            "blocks_epoch": 1,
            "live_stake": "5721241414066",
            "live_size": 0.00026787598158730844,
            "live_saturation": 0.07458075839782029,
            "live_delegators": 181,
            "active_stake": "5727090990610",
            "active_size": 0.0002677054019934437,
            "declared_pledge": "250000000000",
            "live_pledge": "356156149988",
            "margin_cost": 0.049,
            "fixed_cost": "340000000",
            "reward_account": "stake1u98nnlkvkk23vtvf9273uq7cph5ww6u2yq2389psuqet90sv4xv9v",
            "owners": [
                "stake1u98nnlkvkk23vtvf9273uq7cph5ww6u2yq2389psuqet90sv4xv9v"
            ],
            "registration": [
                "a96c79773b7506211eb56bf94886a2face17657d1009f52fb5ea05f19cc8823e",
                "68b302b1a31a47a4688320635c77440f6d5c2845484f1751ac19eb4f361416c6"
            ],
            "retirement": [],
            "calidus_key": null
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
