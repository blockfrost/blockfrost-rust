use crate::*;
use crate::*;
use serde::{Deserialize, Serialize};

impl BlockFrostApi {
    pub async fn nutlink_address(&self, address: &str) -> Result<NutlinkAddress> {
        self.call_endpoint(format!("/nutlink/{}", address).as_str())
            .await
    }

    pub async fn nutlink_address_tickers(
        &self,
        address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<NutlinkAddressTicker>> {
        self.call_paged_endpoint(format!("/nutlink/{}/tickers", address).as_str(), pagination)
            .await
    }

    pub async fn nutlink_address_ticker_by_id(
        &self,
        address: &str,
        ticker: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<NutlinkAddressTicker>> {
        self.call_paged_endpoint(
            format!("/nutlink/{}/tickers/{}", address, ticker).as_str(),
            pagination,
        )
        .await
    }

    pub async fn nutlink_ticker_by_id(
        &self,
        ticker: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<NutlinkTicker>> {
        self.call_paged_endpoint(format!("/nutlink/tickers/{}", ticker).as_str(), pagination)
            .await
    }
}

/// Created by [`nutlink_address`](BlockFrostApi::nutlink_address) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NutlinkAddress {
    /// Bech32 encoded address.
    pub address: String,
    /// URL of the specific metadata file.
    pub metadata_url: String,
    /// Hash of the metadata file.
    pub metadata_hash: String,
    /// The cached metadata of the `metadata_url` file.
    pub metadata: Option<JsonMap>,
}

/// Created by [`nutlink_address_tickers`](BlockFrostApi::nutlink_address_tickers) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NutlinkAddressTicker {
    /// Name of the ticker.
    pub name: String,
    /// Number of ticker record.
    pub count: Integer,
    /// Block height of the latest record.
    pub latest_block: Integer,
}

/// Created by [`nutlink_address_ticker_by_id`](BlockFrostApi::nutlink_address_ticker_by_id) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NutlinkAddressTickerDetailed {
    /// Hash of the transaction.
    pub tx_hash: String,
    /// Block height of the record.
    pub block_height: Integer,
    /// Transaction index within the block.
    pub tx_index: Integer,
    /// Content of the ticker.
    pub payload: JsonValue,
}

/// Created by [`nutlink_ticker_by_id`](BlockFrostApi::nutlink_ticker_by_id) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NutlinkTicker {
    /// Address of a metadata oracle.
    pub address: String,
    /// Hash of the transaction.
    pub tx_hash: String,
    /// Block height of the record.
    pub block_height: Integer,
    /// Transaction index within the block.
    pub tx_index: Integer,
    /// Content of the ticker.
    pub payload: JsonValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_nutlink_address, NutlinkAddress, r#"
    {
      "address": "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz",
      "metadata_url": "https://nut.link/metadata.json",
      "metadata_hash": "6bf124f217d0e5a0a8adb1dbd8540e1334280d49ab861127868339f43b3948af",
      "metadata": {}
    }
    "#}

    test_example! { test_nutlink_address_tickers, Vec<NutlinkAddressTicker>, r#"
    [
      {
        "name": "ADAUSD",
        "count": 1980038,
        "latest_block": 2657092
      },
      {
        "name": "ADAEUR",
        "count": 1980038,
        "latest_block": 2657092
      },
      {
        "name": "ADABTC",
        "count": 1980038,
        "latest_block": 2657092
      }
    ]
    "#}

    test_example! { test_nutlink_address_ticker_detailed, Vec<NutlinkAddressTickerDetailed>, r#"
    [
      {
        "tx_hash": "e8073fd5318ff43eca18a852527166aa8008bee9ee9e891f585612b7e4ba700b",
        "block_height": 2657092,
        "tx_index": 8,
        "payload": [
          {
            "source": "coinGecko",
            "value": "1.29"
          },
          {
            "source": "cryptoCompare",
            "value": "1.283"
          }
        ]
      }
    ]
    "#}
}
