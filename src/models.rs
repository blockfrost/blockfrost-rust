use serde::{Deserialize, Serialize};

macro_rules! test_schema {
    { $test_name:tt, $schema_name:ty, $text:expr } => {
        #[test]
        fn $test_name() {
            let parsed = serde_json::from_str::<$schema_name>($text);
            // Should succeed
            parsed.unwrap();
        }
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub url: String,
    pub version: String,
}

test_schema! { test_root, Root, r#"
{
  "url": "https://blockfrost.io/",
  "version": "0.1.0"
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Health {
    pub is_healthy: bool,
}

test_schema! { test_health, Health, r#"
{
  "is_healthy": true
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HealthClock {
    pub server_time: u64,
}

test_schema! { test_health_clock, HealthClock, r#"
{
  "server_time": 1603400958947
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metric {
    pub time: u64,
    pub calls: u64,
}

test_schema! { test_metrics, Vec<Metric>, r#"
[
  {
    "time": 1612543884,
    "calls": 42
  },
  {
    "time": 1614523884,
    "calls": 6942
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetricEndpoint {
    pub time: u64,
    pub calls: u64,
    pub endpoint: String,
}

test_schema! { test_metrics_endpoints, Vec<MetricEndpoint>, r#"
[
  {
    "time": 1612543814,
    "calls": 182,
    "endpoint": "block"
  },
  {
    "time": 1612543814,
    "calls": 42,
    "endpoint": "epoch"
  },
  {
    "time": 1612543812,
    "calls": 775,
    "endpoint": "block"
  },
  {
    "time": 1612523884,
    "calls": 4,
    "endpoint": "epoch"
  },
  {
    "time": 1612553884,
    "calls": 89794,
    "endpoint": "block"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub stake_address: String,
    pub active: bool,
    pub active_epoch: u64,
    pub controlled_amount: String,
    pub rewards_sum: String,
    pub withdrawals_sum: String,
    pub reserves_sum: String,
    pub treasury_sum: String,
    pub withdrawable_amount: String,
    pub pool_id: String,
}

test_schema! { test_account, Account, r#"
{
  "stake_address": "stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7",
  "active": true,
  "active_epoch": 412,
  "controlled_amount": "619154618165",
  "rewards_sum": "319154618165",
  "withdrawals_sum": "12125369253",
  "reserves_sum": "319154618165",
  "treasury_sum": "12000000",
  "withdrawable_amount": "319154618165",
  "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRewards {
    pub epoch: u64,
    pub amount: String,
    pub pool_id: String,
}

test_schema! { test_account_rewards, Vec<AccountRewards>, r#"
[
  {
    "epoch": 215,
    "amount": "12695385",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  },
  {
    "epoch": 216,
    "amount": "3586329",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  },
  {
    "epoch": 217,
    "amount": "0",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  },
  {
    "epoch": 218,
    "amount": "1395265",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountHistory {
    pub active_epoch: u64,
    pub amount: String,
    pub pool_id: String,
}

test_schema! { test_account_history, Vec<AccountHistory>, r#"
[
  {
    "active_epoch": 210,
    "amount": "12695385",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  },
  {
    "active_epoch": 211,
    "amount": "22695385",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDelegations {
    pub active_epoch: u64,
    pub tx_hash: String,
    pub amount: String,
    pub pool_id: String,
}

test_schema! { test_account_delegations, Vec<AccountDelegations>, r#"
[
  {
    "active_epoch": 210,
    "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
    "amount": "12695385",
    "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
  },
  {
    "active_epoch": 242,
    "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
    "amount": "12691385",
    "pool_id": "pool1kchver88u3kygsak8wgll7htr8uxn5v35lfrsyy842nkscrzyvj"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRegistrations {
    pub tx_hash: String,
    pub action: String,
}

test_schema! { test_account_registrations, Vec<AccountRegistrations>, r#"
[
  {
    "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
    "action": "registered"
  },
  {
    "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
    "action": "deregistered"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountWithdrawals {
    pub tx_hash: String,
    pub amount: String,
}

test_schema! { test_account_withdrawals, Vec<AccountWithdrawals>, r#"
[
  {
    "tx_hash": "48a9625c841eea0dd2bb6cf551eabe6523b7290c9ce34be74eedef2dd8f7ecc5",
    "amount": "454541212442"
  },
  {
    "tx_hash": "4230b0cbccf6f449f0847d8ad1d634a7a49df60d8c142bb8cc2dbc8ca03d9e34",
    "amount": "97846969"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountMirs {
    pub tx_hash: String,
    pub amount: String,
}

test_schema! { test_account_mirs, Vec<AccountMirs>, r#"
[
  {
    "tx_hash": "69705bba1d687a816ff5a04ec0c358a1f1ef075ab7f9c6cc2763e792581cec6d",
    "amount": "2193707473"
  },
  {
    "tx_hash": "baaa77b63d4d7d2bb3ab02c9b85978c2092c336dede7f59e31ad65452d510c13",
    "amount": "14520198574"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountAdresses {
    pub address: String,
}

test_schema! { test_account_addresses, Vec<AccountAdresses>, r#"
[
  {
    "address": "addr1qx2kd28nq8ac5prwg32hhvudlwggpgfp8utlyqxu6wqgz62f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sy0f4qd"
  },
  {
    "address": "addr1qys3czp8s9thc6u2fqed9yq3h24nyw28uk0m6mkgn9dkckjf79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9suth4w4"
  },
  {
    "address": "addr1q8j55h253zcvl326sk5qdt2n8z7eghzspe0ekxgncr796s2f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sjmd35m"
  },
  {
    "address": "addr1q8f7gxrprank3drhx8k5grlux7ene0nlwun8y9thu8mc3yjf79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sls6vnt"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountAdressesAssets {
    pub unit: String,
    pub quantity: String,
}

test_schema! { test_account_addresses_assets, Vec<AccountAdressesAssets>, r#"
[
  {
    "unit": "d5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc537061636542756433343132",
    "quantity": "1"
  },
  {
    "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
    "quantity": "125"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Adresses {
    pub address: String,
    pub amount: Vec<AccountAdressesAssets>,
    pub stake_address: String,
    // Needs to be renamed because type is a reserved word
    #[serde(rename = "type")]
    pub type_: String,
}

test_schema! { test_adresses, Adresses, r#"
{
  "address": "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz",
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
  "stake_address": "stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7",
  "type": "shelley"
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdressesTotal {
    pub address: String,
    pub received_sum: Vec<AccountAdressesAssets>,
    pub sent_sum: Vec<AccountAdressesAssets>,
    pub tx_count: u64,
}

test_schema! { test_adresses_total, AdressesTotal, r#"
{
  "address": "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz",
  "received_sum": [
    {
      "unit": "lovelace",
      "quantity": "42000000"
    },
    {
      "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
      "quantity": "12"
    }
  ],
  "sent_sum": [
    {
      "unit": "lovelace",
      "quantity": "42000000"
    },
    {
      "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
      "quantity": "12"
    }
  ],
  "tx_count": 12
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdressesUtxos {
    pub tx_hash: String,
    pub output_index: u64,
    pub amount: Vec<AccountAdressesAssets>,
    pub block: String,
}

test_schema! { test_adresses_utxos, Vec<AdressesUtxos>, r#"
[
  {
    "tx_hash": "39a7a284c2a0948189dc45dec670211cd4d72f7b66c5726c08d9b3df11e44d58",
    "output_index": 0,
    "amount": [
      {
        "unit": "lovelace",
        "quantity": "42000000"
      }
    ],
    "block": "7eb8e27d18686c7db9a18f8bbcfe34e3fed6e047afaa2d969904d15e934847e6"
  },
  {
    "tx_hash": "4c4e67bafa15e742c13c592b65c8f74c769cd7d9af04c848099672d1ba391b49",
    "output_index": 0,
    "amount": [
      {
        "unit": "lovelace",
        "quantity": "729235000"
      }
    ],
    "block": "953f1b80eb7c11a7ffcd67cbd4fde66e824a451aca5a4065725e5174b81685b7"
  },
  {
    "tx_hash": "768c63e27a1c816a83dc7b07e78af673b2400de8849ea7e7b734ae1333d100d2",
    "output_index": 1,
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
    "block": "5c571f83fe6c784d3fbc223792627ccf0eea96773100f9aedecf8b1eda4544d7"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdressesTxs(String);

test_schema! { test_adresses_txs, Vec<AdressesTxs>, r#"
[
  "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
  "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0"
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdressesTransactions {
    pub tx_hash: String,
    pub tx_index: u64,
    pub block_height: u64,
}

test_schema! { test_adresses_transactions, Vec<AdressesTransactions>, r#"
[
  {
    "tx_hash": "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
    "tx_index": 6,
    "block_height": 69
  },
  {
    "tx_hash": "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
    "tx_index": 9,
    "block_height": 4547
  },
  {
    "tx_hash": "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b",
    "tx_index": 0,
    "block_height": 564654
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assets {
    pub asset: String,
    pub quantity: String,
}

test_schema! { test_assets, Vec<Assets>, r#"
[
  {
    "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
    "quantity": "1"
  },
  {
    "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e75d",
    "quantity": "100000"
  },
  {
    "asset": "6804edf9712d2b619edb6ac86861fe93a730693183a262b165fcc1ba1bc99cad",
    "quantity": "18605647"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnchainMetadata {
    pub name: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub ticker: String,
    pub url: String,
    pub logo: String,
    pub decimals: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsAsset {
    pub asset: String,
    pub policy_id: String,
    pub asset_name: String,
    pub fingerprint: String,
    pub quantity: String,
    pub initial_mint_tx_hash: String,
    pub mint_or_burn_count: u64,
    pub onchain_metadata: OnchainMetadata,
    pub metadata: Metadata,
}

test_schema! { test_assets_asset, AssetsAsset, r#"
{
  "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
  "policy_id": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a7",
  "asset_name": "6e7574636f696e",
  "fingerprint": "asset1pkpwyknlvul7az0xx8czhl60pyel45rpje4z8w",
  "quantity": "12000",
  "initial_mint_tx_hash": "6804edf9712d2b619edb6ac86861fe93a730693183a262b165fcc1ba1bc99cad",
  "mint_or_burn_count": 1,
  "onchain_metadata": {
    "name": "My NFT token",
    "image": "ipfs://ipfs/QmfKyJ4tuvHowwKQCbCHj4L5T3fSj8cjs7Aau8V7BWv226"
  },
  "metadata": {
    "name": "nutcoin",
    "description": "The Nut Coin",
    "ticker": "nutc",
    "url": "https://www.stakenuts.com/",
    "logo": "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",
    "decimals": 6
  }
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsHistory {
    pub tx_hash: String,
    pub amount: String,
    pub action: String,
}

test_schema! { test_assets_history, Vec<AssetsHistory>, r#"
[
  {
    "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
    "amount": "10",
    "action": "minted"
  },
  {
    "tx_hash": "9c190bc1ac88b2ab0c05a82d7de8b71b67a9316377e865748a89d4426c0d3005",
    "amount": "5",
    "action": "burned"
  },
  {
    "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
    "amount": "5",
    "action": "burned"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsTxs(String);

test_schema! { test_assets_txs, Vec<AssetsTxs>, r#"
[
  "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
  "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
  "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsTransactions {
    pub tx_hash: String,
    pub tx_index: u64,
    pub block_height: u64,
}

test_schema! { test_assets_transactions, Vec<AssetsTransactions>, r#"
[
  {
    "tx_hash": "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
    "tx_index": 6,
    "block_height": 69
  },
  {
    "tx_hash": "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
    "tx_index": 9,
    "block_height": 4547
  },
  {
    "tx_hash": "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b",
    "tx_index": 0,
    "block_height": 564654
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsAdresses {
    pub address: String,
    pub quantity: String,
}

test_schema! { test_assets_adresses, Vec<AssetsAdresses>, r#"
[
  {
    "address": "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz",
    "quantity": "1"
  },
  {
    "address": "addr1qyhr4exrgavdcn3qhfcc9f939fzsch2re5ry9cwvcdyh4x4re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qdpvhza",
    "quantity": "100000"
  },
  {
    "address": "addr1q8zup8m9ue3p98kxlxl9q8rnyan8hw3ul282tsl9s326dfj088lvedv4zckcj24arcpasr0gua4c5gq4zw2rpcpjk2lq8cmd9l",
    "quantity": "18605647"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetsPolicy {
    pub asset: String,
    pub quantity: String,
}

test_schema! { test_assets_policy, Vec<AssetsPolicy>, r#"
[
  {
    "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
    "quantity": "1"
  },
  {
    "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a766e",
    "quantity": "100000"
  },
  {
    "asset": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb574636f696e",
    "quantity": "18605647"
  }
]
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub time: u64,
    pub height: u64,
    pub hash: String,
    pub slot: u64,
    pub epoch: u64,
    pub epoch_slot: u64,
    pub slot_leader: String,
    pub size: u64,
    pub tx_count: u64,
    pub output: String,
    pub fees: String,
    pub block_vrf: String,
    pub previous_block: String,
    pub next_block: String,
    pub confirmations: u64,
}

test_schema! { test_blocks_latest, Block, r#"
{
  "time": 1641338934,
  "height": 15243593,
  "hash": "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
  "slot": 412162133,
  "epoch": 425,
  "epoch_slot": 12,
  "slot_leader": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
  "size": 3,
  "tx_count": 1,
  "output": "128314491794",
  "fees": "592661",
  "block_vrf": "vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
  "previous_block": "43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
  "next_block": "8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
  "confirmations": 4698
}
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockLatestTxs(String);

test_schema! { test_blocks_latest_txs, Vec<BlockLatestTxs>, r#"
[
  "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
  "4eef6bb7755d8afbeac526b799f3e32a624691d166657e9d862aaeb66682c036",
  "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
  "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b"
]
"# }
// {
//   "time": 1641338934,
//   "height": 15243593,
//   "hash": "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
//   "slot": 412162133,
//   "epoch": 425,
//   "epoch_slot": 12,
//   "slot_leader": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
//   "size": 3,
//   "tx_count": 1,
//   "output": "128314491794",
//   "fees": "592661",
//   "block_vrf": "vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
//   "previous_block": "43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
//   "next_block": "8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
//   "confirmations": 4698
// }

//   "time": 1641338934,
//   "height": 15243593,
//   "hash": "4ea1ba291e8eef538635a53e59fddba7810d1679631cc3aed7c8e6c4091a516a",
//   "slot": 412162133,
//   "epoch": 425,
//   "epoch_slot": 12,
//   "slot_leader": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2qnikdy",
//   "size": 3,
//   "tx_count": 1,
//   "output": "128314491794",
//   "fees": "592661",
//   "block_vrf": "vrf_vk1wf2k6lhujezqcfe00l6zetxpnmh9n6mwhpmhm0dvfh3fxgmdnrfqkms8ty",
//   "previous_block": "43ebccb3ac72c7cebd0d9b755a4b08412c9f5dcb81b8a0ad1e3c197d29d47b05",
//   "next_block": "8367f026cf4b03e116ff8ee5daf149b55ba5a6ec6dec04803b8dc317721d15fa",
//   "confirmations": 4698

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Epoch {
    pub epoch: u64,
    pub start_time: u64,
    pub end_time: u64,
    pub first_block_time: u64,
    pub last_block_time: u64,
    pub block_count: u64,
    pub tx_count: u64,
    pub output: String,
    pub fees: String,
    pub active_stake: String,
}

test_schema! { test_epoch, Epoch, r#"
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
"# }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochParameters {
    pub epoch: u64,
    pub min_fee_a: u64,
    pub min_fee_b: u64,
    pub max_block_size: u64,
    pub max_tx_size: u64,
    pub max_block_header_size: u64,
    pub key_deposit: String,
    pub pool_deposit: String,
    pub e_max: u64,
    pub n_opt: u64,
    pub a0: u64,
    pub rho: u64,
    pub tau: u64,
    pub decentralisation_param: u64,
    pub extra_entropy: u64,
    pub protocol_major_ver: u64,
    pub protocol_minor_ver: u64,
    pub min_utxo: String,
    pub min_pool_cost: String,
    pub nonce: String,
}

// test_schema! { test_epoch_parameters, EpochParameters, r#"
// {
//   "epoch": 225,
//   "min_fee_a": 44,
//   "min_fee_b": 155381,
//   "max_block_size": 65536,
//   "max_tx_size": 16384,
//   "max_block_header_size": 1100,
//   "key_deposit": "2000000",
//   "pool_deposit": "500000000",
//   "e_max": 18,
//   "n_opt": 150,
//   "a0": 0.3,
//   "rho": 0.003,
//   "tau": 0.2,
//   "decentralisation_param": 0.5,
//   "extra_entropy": null,
//   "protocol_major_ver": 2,
//   "protocol_minor_ver": 0,
//   "min_utxo": "1000000",
//   "min_pool_cost": "340000000",
//   "nonce": "1a3be38bcbb7911969283716ad7aa550250226b76a61fc51cc9a9a35d9276d81"
// }
// "# }
