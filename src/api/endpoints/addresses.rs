use blockfrost_openapi::models::{
    address_content::AddressContent, address_content_extended::AddressContentExtended,
    address_content_total::AddressContentTotal,
    address_transactions_content_inner::AddressTransactionsContentInner,
    address_utxo_content_inner::AddressUtxoContentInner,
};

use crate::*;

impl BlockfrostAPI {
    pub async fn addresses(&self, address: &str) -> BlockfrostResult<AddressContent> {
        self.call_endpoint(format!("/addresses/{}", address).as_str())
            .await
    }

    pub async fn addresses_extended(
        &self, address: &str,
    ) -> BlockfrostResult<AddressContentExtended> {
        self.call_endpoint(format!("/addresses/{}/extended", address).as_str())
            .await
    }

    pub async fn addresses_total(&self, address: &str) -> BlockfrostResult<AddressContentTotal> {
        self.call_endpoint(format!("/addresses/{}/total", address).as_str())
            .await
    }

    pub async fn addresses_utxos(
        &self, address: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<AddressUtxoContentInner>> {
        self.call_paged_endpoint(format!("/addresses/{}/utxos", address).as_str(), pagination)
            .await
    }

    pub async fn addresses_utxos_asset(
        &self, address: &str, asset: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<AddressUtxoContentInner>> {
        self.call_paged_endpoint(
            format!("/addresses/{}/utxos/{}", address, asset).as_str(),
            pagination,
        )
        .await
    }

    pub async fn addresses_transactions(
        &self, address: &str, pagination: Pagination,
    ) -> BlockfrostResult<Vec<AddressTransactionsContentInner>> {
        self.call_paged_endpoint(
            format!("/addresses/{}/transactions", address).as_str(),
            pagination,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockfrost_openapi::models::tx_content_output_amount_inner::TxContentOutputAmountInner;
    use serde_json::json;

    #[tokio::test]
    async fn test_address() {
        let json_value = json!({
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
            "type": "shelley",
            "script": false
        });
        serde_json::from_value::<AddressContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_address_total() {
        let json_value = json!({
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
        });

        serde_json::from_value::<AddressContentTotal>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_address_utxo() {
        let json_value = json!(  [
          {
            "address": "hm",
            "tx_index": 2,
            "data_hash": null,
            "tx_hash": "39a7a284c2a0948189dc45dec670211cd4d72f7b66c5726c08d9b3df11e44d58",
            "output_index": 0,
            "inline_datum": "0x68656c6c6f",
            "reference_script_hash": null,
            "amount": [
              {
                "unit": "lovelace",
                "quantity": "42000000"
              }
            ],
            "block": "7eb8e27d18686c7db9a18f8bbcfe34e3fed6e047afaa2d969904d15e934847e6"
          },
          {
            "address": "hm2",
            "tx_index": 2,
            "data_hash": null,
            "tx_hash": "4c4e67bafa15e742c13c592b65c8f74c769cd7d9af04c848099672d1ba391b49",
            "output_index": 0,
            "reference_script_hash": null,
            "inline_datum": "0x68656c6c6f",
            "amount": [
              {
                "unit": "lovelace",
                "quantity": "729235000"
              }
            ],
            "block": "953f1b80eb7c11a7ffcd67cbd4fde66e824a451aca5a4065725e5174b81685b7"
          },
          {
            "address": "hm3",
            "tx_hash": "768c63e27a1c816a83dc7b07e78af673b2400de8849ea7e7b734ae1333d100d2",
            "output_index": 1,
            "reference_script_hash": null,
            "data_hash": null,
            "inline_datum": "0x68656c6c6f",
            "tx_index": 2,
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
        ]);

        serde_json::from_value::<Vec<AddressUtxoContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_address_transaction() {
        let json_value = json!([
          {
            "tx_hash": "8788591983aa73981fc92d6cddbbe643959f5a784e84b8bee0db15823f575a5b",
            "tx_index": 6,
            "block_height": 69,
            "block_time": 4547
          },
          {
            "tx_hash": "52e748c4dec58b687b90b0b40d383b9fe1f24c1a833b7395cdf07dd67859f46f",
            "tx_index": 9,
            "block_height": 4547,
             "block_time": 42
          },
          {
            "tx_hash": "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b",
            "tx_index": 0,
            "block_height": 564654,
             "block_time": 666
          }
        ]);
        serde_json::from_value::<Vec<AddressTransactionsContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_transaction_amount() {
        let json_value = json!({
            "unit": "lovelace",
            "quantity": "42000000"
        });

        serde_json::from_value::<TxContentOutputAmountInner>(json_value).unwrap();
    }
}
