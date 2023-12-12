use crate::{pagination::Pagination, *};
use blockfrost_openapi::models::{
    nutlink_address::NutlinkAddress, nutlink_address_ticker_inner::NutlinkAddressTickerInner,
    nutlink_address_ticker_inner_payload::NutlinkAddressTickerInnerPayload,
    nutlink_address_tickers_inner::NutlinkAddressTickersInner,
};

impl BlockfrostAPI {
    pub async fn nutlink_address(&self, address: &str) -> BlockfrostResult<NutlinkAddress> {
        self.call_endpoint(format!("/nutlink/{}", address).as_str())
            .await
    }

    pub async fn nutlink_address_tickers(
        &self,
        address: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<NutlinkAddressTickersInner>> {
        self.call_paged_endpoint(format!("/nutlink/{}/tickers", address).as_str(), pagination)
            .await
    }

    pub async fn nutlink_address_ticker_by_id(
        &self,
        address: &str,
        ticker: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<NutlinkAddressTickerInnerPayload>> {
        self.call_paged_endpoint(
            format!("/nutlink/{}/tickers/{}", address, ticker).as_str(),
            pagination,
        )
        .await
    }

    pub async fn nutlink_ticker_by_id(
        &self,
        ticker: &str,
        pagination: Pagination,
    ) -> BlockfrostResult<Vec<NutlinkAddressTickerInner>> {
        self.call_paged_endpoint(format!("/nutlink/tickers/{}", ticker).as_str(), pagination)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blockfrost_openapi::models::nutlink_address_tickers_inner::NutlinkAddressTickersInner;
    use serde_json::json;

    #[tokio::test]
    async fn test_nutlink_address() {
        let json_value = json!({
            "address": "addr1qxqs59lphg8g6qndelq8xwqn60ag3aeyfcp33c2kdp46a09re5df3pzwwmyq946axfcejy5n4x0y99wqpgtp2gd0k09qsgy6pz",
            "metadata_url": "https://nut.link/metadata.json",
            "metadata_hash": "6bf124f217d0e5a0a8adb1dbd8540e1334280d49ab861127868339f43b3948af",
            "metadata": {}
        });

        serde_json::from_value::<NutlinkAddress>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_nutlink_address_tickers() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<NutlinkAddressTickersInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_nutlink_address_ticker_detailed() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<NutlinkAddressTickerInner>>(json_value).unwrap();
    }
}
