use crate::{pagination::Pagination, *};
use blockfrost_openapi::models::{
    asset_addresses_inner::AssetAddressesInner, asset_history_inner::AssetHistoryInner,
    asset_policy_inner::AssetPolicyInner, asset_transactions_inner::AssetTransactionsInner,
    assets_inner::AssetsInner,
};

impl BlockfrostAPI {
    pub async fn assets_by_id(&self, asset: &str) -> BlockfrostResult<AssetsInner> {
        self.call_endpoint(format!("/assets/{}", asset).as_str())
            .await
    }

    pub async fn assets(&self, pagination: Pagination) -> BlockfrostResult<Vec<AssetsInner>> {
        self.call_paged_endpoint("/assets", pagination).await
    }

    pub async fn assets_history(
        &self,
        asset: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<AssetHistoryInner>> {
        self.call_paged_endpoint(format!("/assets/{}/history", asset).as_str(), pagination)
            .await
    }

    pub async fn assets_transactions(
        &self,
        asset: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<AssetTransactionsInner>> {
        self.call_paged_endpoint(
            format!("/assets/{}/transactions", asset).as_str(),
            pagination,
        )
        .await
    }

    pub async fn assets_addresses(
        &self,
        asset: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<AssetAddressesInner>> {
        self.call_paged_endpoint(format!("/assets/{}/addresses", asset).as_str(), pagination)
            .await
    }

    pub async fn assets_policy_by_id(
        &self,
        policy_id: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<AssetPolicyInner>> {
        self.call_paged_endpoint(format!("/assets/policy/{}", policy_id).as_str(), pagination)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockfrost_openapi::models::{
        asset::Asset, asset_transactions_inner::AssetTransactionsInner,
    };
    use serde_json::json;

    #[tokio::test]
    async fn test_asset() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AssetsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_asset_details() {
        let json_value = json!({
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
        });

        serde_json::from_value::<Asset>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_asset_history() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AssetHistoryInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_asset_transaction() {
        let json_value = json!([
            {
                "tx_hash": "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
                "tx_index": 6,
                "block_height": 69,
                "block_time": 13
            },
            {
                "tx_hash": "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
                "tx_index": 9,
                "block_height": 4547,
                "block_time": 11
            },
            {
                "tx_hash": "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b",
                "tx_index": 0,
                "block_height": 564654,
                "block_time": 12
            }
        ]);

        serde_json::from_value::<Vec<AssetTransactionsInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_asset_address() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AssetAddressesInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_asset_policy_by_id() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AssetPolicyInner>>(json_value).unwrap();
    }
}
