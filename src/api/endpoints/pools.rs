use crate::*;
use serde::{Deserialize, Serialize};

impl BlockFrostApi {
    endpoints! {
        /// List of registered stake pools.
        pools() -> Vec<String> => "/pools";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools/get"),
        /// List of already retired pools.
        pools_retired() -> Vec<RetiredPool> => "/pools/retired";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1retired/get"),
        /// List of retiring stake pools.
        pools_retiring() -> Vec<RetiringPool> => "/pools/retired";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1retiring/get"),
        /// Pool information.
        pools_by_id(pool_id: &str) -> Pool => "/pools/{pool_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}/get"),
        /// History of stake pool parameters over epochs.
        pools_history(pool_id: &str) -> Vec<PoolHistory> => "/pools/{pool_id}/history";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1history/get"),
        /// Stake pool registration metadata.
        pools_metadata(pool_id: &str) -> PoolMetadata => "/pools/{pool_id}/metadata";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1metadata/get"),
        /// Relays of a stake pool.
        pools_relays(pool_id: &str) -> Vec<PoolRelay> => "/pools/{pool_id}/relays";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1relays/get"),
        /// List of current stake pools delegators.
        pools_delegators(pool_id: &str) -> Vec<PoolRelay> => "/pools/{pool_id}/delegators";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1delegators/get"),
        /// List of stake pool blocks.
        pools_blocks(pool_id: &str) -> Vec<String> => "/pools/{pool_id}/delegators";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1blocks/get"),
        /// List of certificate updates to the stake pool.
        pools_updates(pool_id: &str) -> Vec<PoolUpdate> => "/pools/{pool_id}/updates";
            ("https://docs.blockfrost.io/#tag/Cardano-Pools/paths/~1pools~1{pool_id}~1updates/get"),
    }
}

/// Created by [`pools_retired`](BlockFrostApi::pools_retired) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetiredPool {
    /// Bech32 encoded pool ID.
    pub pool_id: String,
    /// Retirement epoch number.
    pub epoch: Integer,
}

/// Created by [`pools_retiring`](BlockFrostApi::pools_retiring) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RetiringPool {
    /// Bech32 encoded pool ID.
    pub pool_id: String,
    /// Retirement epoch number.
    pub epoch: Integer,
}

/// Created by [`pools_by_id`](BlockFrostApi::pools_by_id) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pool {
    /// Bech32 pool ID.
    pub pool_id: String,
    /// Hexadecimal pool ID.
    pub hex: String,
    /// VRF key hash.
    pub vrf_key: String,
    /// Total minted blocks.
    pub blocks_minted: Integer,
    pub live_stake: String,
    pub live_size: Float,
    pub live_saturation: Float,
    pub live_delegators: Integer,
    pub active_stake: String,
    pub active_size: Float,
    /// Stake pool certificate pledge.
    pub declared_pledge: String,
    /// Stake pool urrent pledge.
    pub live_pledge: String,
    /// Margin tax cost of the stake pool.
    pub margin_cost: Float,
    /// Fixed tax cost of the stake pool.
    pub fixed_cost: String,
    /// Bech32 reward account of the stake pool.
    pub reward_account: String,
    pub owners: Vec<String>,
    pub registration: Vec<String>,
    pub retirement: Vec<String>,
}

/// Created by [`pools_history`](BlockFrostApi::pools_history) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolHistory {
    /// Epoch number.
    pub epoch: Integer,
    /// Number of blocks created by pool.
    pub blocks: Integer,
    /// Active (Snapshot of live stake 2 epochs ago) stake in Lovelaces.
    pub active_stake: String,
    /// Pool size (percentage) of overall active stake at that epoch.
    pub active_size: Float,
    /// Number of delegators for epoch.
    pub delegators_count: Integer,
    /// Total rewards received before distribution to delegators.
    pub rewards: String,
    /// Pool operator rewards.
    pub fees: String,
}

/// Created by [`pools_metadata`](BlockFrostApi::pools_metadata) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolMetadata {
    /// Bech32 pool ID.
    pub pool_id: String,
    /// Hexadecimal pool ID.
    pub hex: String,
    /// URL to the stake pool metadata.
    pub url: Option<String>,
    /// Hash of the metadata file.
    pub hash: Option<String>,
    /// Ticker of the stake pool.
    pub ticker: Option<String>,
    /// Name of the stake pool.
    pub name: Option<String>,
    /// Description of the stake pool.
    pub description: Option<String>,
    /// Home page of the stake pool.
    pub homepage: Option<String>,
}

/// Created by [`pools_relays`](BlockFrostApi::pools_relays) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolRelay {
    /// IPv4 address of the relay.
    pub ipv4: Option<String>,
    /// IPv6 address of the relay.
    pub ipv6: Option<String>,
    /// DNS name of the relay.
    pub dns: Option<String>,
    /// DNS SRV entry of the relay.
    pub dns_srv: Option<String>,
    /// Network port of the relay.
    pub port: Integer,
}

/// Created by [`pools_delegators`](BlockFrostApi::pools_delegators) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolDelegator {
    /// Bech32 encoded stake addresses.
    pub address: String,
    /// Currently delegated amount.
    pub live_stake: String,
}

/// Created by [`pools_updates`](BlockFrostApi::pools_updates) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PoolUpdate {
    /// Transaction ID.
    pub tx_hash: String,
    /// Certificate within the transaction.
    pub cert_index: Integer,
    /// Action in the certificate.
    pub action: ActionType, // "registered" | "deregistered"
}

#[cfg(test)]
mod tests {
    use super::*;

    test_schema! { test_pools, Vec<String>, r#"
    [
      "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
      "pool1hn7hlwrschqykupwwrtdfkvt2u4uaxvsgxyh6z63703p2knj288",
      "pool1ztjyjfsh432eqetadf82uwuxklh28xc85zcphpwq6mmezavzad2"
    ]
    "# }

    test_schema! { test_pools_retired, Vec<RetiredPool>, r#"
    [
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
    ]
    "# }

    test_schema! { test_pools_retiring, Vec<RetiringPool>, r#"
    [
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
    ]
    "# }

    test_schema! { test_pools_by_id, Pool, r#"
    {
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
    }
    "# }

    test_schema! { test_pools_history, Vec<PoolHistory>, r#"
    [
      {
        "epoch": 233,
        "blocks": 22,
        "active_stake": "20485965693569",
        "active_size": 1.2345,
        "delegators_count": 115,
        "rewards": "206936253674159",
        "fees": "1290968354"
      }
    ]
    "# }

    test_schema! { test_pools_metadata, PoolMetadata, r#"
    {
      "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
      "hex": "0f292fcaa02b8b2f9b3c8f9fd8e0bb21abedb692a6d5058df3ef2735",
      "url": "https://stakenuts.com/mainnet.json",
      "hash": "47c0c68cb57f4a5b4a87bad896fc274678e7aea98e200fa14a1cb40c0cab1d8c",
      "ticker": "NUTS",
      "name": "Stake Nuts",
      "description": "The best pool ever",
      "homepage": "https://stakentus.com/"
    }
    "# }

    test_schema! { test_pools_relays, Vec<PoolRelay>, r#"
    [
      {
        "ipv4": "4.4.4.4",
        "ipv6": "https://stakenuts.com/mainnet.json",
        "dns": "relay1.stakenuts.com",
        "dns_srv": "_relays._tcp.relays.stakenuts.com",
        "port": 3001
      }
    ]
    "# }

    test_schema! { test_pool_delegators, Vec<PoolDelegator>, r#"
    [
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
    ]
    "# }

    test_schema! { test_pools_blocks, Vec<String>, r#"
    [
      "d8982ca42cfe76b747cc681d35d671050a9e41e9cfe26573eb214e94fe6ff21d",
      "026436c539e2ce84c7f77ffe669f4e4bbbb3b9c53512e5857dcba8bb0b4e9a8c",
      "bcc8487f419b8c668a18ea2120822a05df6dfe1de1f0fac3feba88cf760f303c",
      "86bf7b4a274e0f8ec9816171667c1b4a0cfc661dc21563f271acea9482b62df7"
    ]
    "# }

    test_schema! { test_pools_updates, Vec<PoolUpdate>, r#"
    [
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
    ]
    "# }
}
