use crate::*;
use blockfrost_openapi::models::{
    pool::Pool, pool_delegators_inner::PoolDelegatorsInner, pool_history_inner::PoolHistoryInner,
    pool_list_retire_inner::PoolListRetireInner, pool_metadata::PoolMetadata,
    pool_updates_inner::PoolUpdatesInner,
    tx_content_pool_certs_inner_relays_inner::TxContentPoolCertsInnerRelaysInner,
};

impl BlockfrostAPI {
    pub async fn derive_address(
        &self,
        xpub: &str,
        role: &str,
        index: &str,
    ) -> BlockfrostResult<Derive> {
        self.call_endpoint(format!("/utils/addresses/xpub/{}/{}/{}", xpub, role, index).as_str())
            .await
    }

    pub async fn utils_tx_evaluate(&self, transaction_data: Vec<u8>) -> BlockfrostResult<String> {
        let body = Body::from(transaction_data);
        let url = Url::from_endpoint(self.base_url.as_str(), "/utils/txs/evaluate")?;

        let request = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", HeaderValue::from_static("application/json"))
            .body(body);

        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| BlockfrostError::Reqwest {
                url: url.clone(),
                reason,
            })?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }
        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }

    pub async fn utils_tx_evaluate_utxos(
        &self,
        transaction_data: Vec<u8>,
    ) -> BlockfrostResult<String> {
        let body = Body::from(transaction_data);
        let url = Url::from_endpoint(self.base_url.as_str(), "/utils/txs/evaluate/utxos")?;

        let request = self
            .client
            .request(Method::POST, &url)
            .header("Content-Type", HeaderValue::from_static("application/cbor"))
            .body(body);

        let (status, text) = send_request(request, self.settings.retry_settings)
            .await
            .map_err(|reason| BlockfrostError::Reqwest {
                url: url.clone(),
                reason,
            })?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }
        json_from(&text).map_err(|reason| json_error(url, text, reason))
    }
}
