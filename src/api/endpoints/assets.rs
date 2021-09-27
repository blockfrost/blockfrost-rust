use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    endpoints! {
        /// List of assets.
        assets() -> Vec<Asset> => "/assets";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets/get"),

        /// Detailed information about a specific asset.
        assets_by_id(asset: &str) -> AssetDetails => "/assets/{asset}";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets~1{asset}/get"),

        /// History of a specific asset.
        assets_history(asset: &str) -> Vec<AssetHistory> => "/assets/{asset}/history";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets~1{asset}~1history/get"),

        /// List of a specific asset transactions.
        assets_transactions(asset: &str) -> Vec<AssetTransaction> => "/assets/{asset}/transactions";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets~1{asset}~1transactions/get"),

        /// List of a addresses containing a specific asset.
        assets_addresses(asset: &str) -> Vec<AssetAddress> => "/assets/{asset}/addresses";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets~1{asset}~1addresses/get"),

        /// List of asset minted under a specific policy.
        assets_policy_by_id(policy_id: &str) -> Vec<AssetPolicy> => "/assets/policy/{policy_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Assets/paths/~1assets~1policy~1{policy_id}/get"),
    }
}

/// Created by [`assets`](BlockFrostApi::assets) method.
///
/// Similar to [`Amount`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    /// Asset identifier.
    ///
    /// Format: Concatenation of asset `policy_id` and hex-encoded `asset_name`.
    pub asset: String,
    /// Current asset quantity.
    pub quantity: String,
}

/// Created by [`assets_by_id`](BlockFrostApi::assets_by_id) method.
///
/// More detailed version of [`Asset`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDetails {
    /// Hex-encoded asset full name.
    pub asset: String,
    /// Policy ID of the asset.
    pub policy_id: String,
    /// Hex-encoded asset name of the asset.
    pub asset_name: Option<String>,
    /// CIP14 based user-facing fingerprint.
    pub fingerprint: String,
    /// Current asset quantity.
    pub quantity: String,
    /// ID of the initial minting transaction.
    pub initial_mint_tx_hash: String,
    /// Count of mint and burn transactions.
    pub mint_or_burn_count: Integer,
    /// On-chain metadata stored in the minting transaction under label 721, community discussion
    /// around the standard ongoing at <https://github.com/cardano-foundation/CIPs/pull/85>
    pub onchain_metadata: Option<ArbitraryJson>,
    pub metadata: Option<AssetMetadata>,
}

/// Inner member of [`AssetDetails`].
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetMetadata {
    /// Asset name.
    pub name: String,
    /// Asset description.
    pub description: String,
    pub ticker: Option<String>,
    /// Asset website.
    pub url: Option<String>,
    /// Base64 encoded logo of the asset.
    pub logo: Option<String>,
    /// Number of decimal places of the asset unit.
    pub decimals: Option<Integer>,
}

/// Created by [`assets_history`](BlockFrostApi::assets_history) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetHistory {
    /// Hash of the transaction containing the asset action.
    pub tx_hash: String,
    /// Action executed upon the asset policy.
    pub action: String, // "minted" | "burned"
    /// Asset amount of the specific action.
    pub amount: String,
}

/// Created by [`assets_transactions`](BlockFrostApi::assets_transactions) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetTransaction {
    /// Hash of the transaction.
    pub tx_hash: String,
    /// Transaction index within the block.
    pub tx_index: Integer,
    /// Block height.
    pub block_height: Integer,
}

/// Created by [`assets_addresses`](BlockFrostApi::assets_addresses) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetAddress {
    /// Address containing the specific asset.
    pub address: String,
    /// Asset quantity on the specific address.
    pub quantity: String,
}

/// Created by [`assets_addresses`](BlockFrostApi::assets_addresses) and [`assets_policy_by_id`](BlockFrostApi::assets_policy_by_id) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPolicy {
    /// Concatenation of the policy_id and hex-encoded asset_name.
    pub asset: String,
    /// Current asset quantity.
    pub quantity: String,
}

/// Inner enum for [`AssetHistory`].
///
/// Action executed upon the asset policy.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AssetHistoryActionType {
    Minted,
    Burned,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_asset, Vec<Asset>, r#"
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

    test_example! { test_asset_details, AssetDetails, r#"
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
        "image": "ipfs://ipfs/QmfKyJ4tuvHowwKQCbCHj4L5T3fSj8cjs7Aau8V7BWv226",
        "additional_field": "anything"
      },
      "metadata": {
        "name": "nutcoin",
        "description": "The Nut Coin",
        "ticker": "nutc",
        "url": "https://www.stakenuts.com/",
        "logo": "iVBORw0KGgoAAAAAAABGdBTUEAALGPC/xhBQAAAAAASUVORK5CYII=",
        "decimals": 6
      }
    }
    "# }

    test_example! { test_asset_history, Vec<AssetHistory>, r#"
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

    test_example! { test_asset_transaction, Vec<AssetTransaction>, r#"
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

    test_example! { test_asset_address, Vec<AssetAddress>, r#"
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

    test_example! { test_asset_policy_by_id, Vec<AssetPolicy>, r#"
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
}
