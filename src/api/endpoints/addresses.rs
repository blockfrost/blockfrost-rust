use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    endpoints! {
        /// Obtain information about a specific address.
        addresses(address: &str) -> Address => "/addresses/{address}";
            ("https://docs.blockfrost.io/#tag/Cardano-Addresses/paths/~1addresses~1{address}/get"),

        /// Obtain details about an address.
        addresses_total(address: &str) -> AddressTotal => "/addresses/{address}/total";
            ("https://docs.blockfrost.io/#tag/Cardano-Addresses/paths/~1addresses~1{address}~1total/get"),
    }
    paged_endpoints! {
        /// UTXOs of the address.
        addresses_utxos(address: &str) -> Vec<AddressUtxo> => "/addresses/{address}/utxos";
            ("https://docs.blockfrost.io/#tag/Cardano-Addresses/paths/~1addresses~1{address}~1utxos/get"),

        /// Transactions on the address.
        addresses_transactions(address: &str) -> Vec<AddressTransaction> => "/addresses/{address}/transactions";
            ("https://docs.blockfrost.io/#tag/Cardano-Addresses/paths/~1addresses~1{address}~1transactions/get"),
    }
}

/// Created by [`addresses`](BlockFrostApi::addresses) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Address {
    /// Bech32 encoded addresses.
    pub address: String,
    /// Sum of all owned assets.
    pub amount: Vec<Amount>,
    /// Stake address that controls the key.
    pub stake_address: Option<String>,
    /// Address era.
    #[serde(rename = "type")]
    pub type_: AdressType, // "byron" | "shelley"
}

/// Created by [`addresses_total`](BlockFrostApi::addresses_total) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressTotal {
    /// Bech32 encoded addresses.
    pub address: String,
    /// Sum of all transaction received assets.
    pub received_sum: Vec<Amount>,
    /// Sum of all transaction sent assets.
    pub sent_sum: Vec<Amount>,
    /// Count of all transactions on the address.
    pub tx_count: Integer,
}

/// Created by [`addresses_utxos`](BlockFrostApi::addresses_utxos) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressUtxo {
    /// Bech32 encoded addresses - useful when querying by payment_cred
    pub address: String,
    /// Transaction hash of the UTXO.
    pub tx_hash: String,
    /// UTXO index in the transaction.
    pub output_index: u32,
    /// Sum of assets for this UTXO.
    pub amount: Vec<Amount>,
    /// Block hash of the UTXO.
    pub block: String,
    /// The hash of the transaction output datum
    pub data_hash: Option<String>,
    /// CBOR encoded inline datum
    pub inline_datum: Option<String>,
    /// The hash of the reference script of the output
    pub reference_script_hash: Option<String>,
}

/// Created by [`addresses_transactions`](BlockFrostApi::addresses_transactions) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressTransaction {
    /// Hash of the transaction.
    pub tx_hash: String,
    /// Transaction index within the block.
    pub tx_index: Integer,
    /// Block height.
    pub block_height: Integer,
}

/// Inner enum for [`Address`].
///
/// Address era.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AdressType {
    Byron,
    Shelley,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_address, Address, r#"
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

    test_example! { test_address_total, AddressTotal, r#"
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

    test_example! { test_address_utxo, Vec<AddressUtxo>, r#"
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

    test_example! { test_address_transaction, Vec<AddressTransaction>, r#"
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

    test_example! { test_transaction_amount, Amount, r#"
    {
      "unit": "lovelace",
      "quantity": "42000000"
    }
    "# }
}
