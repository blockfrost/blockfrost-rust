use crate::{request::send_request, url::Url, *};
use blockfrost_openapi::models::utils_addresses_xpub::UtilsAddressesXpub;
use reqwest::{header::HeaderValue, Body, Method};
use serde_json::{from_str as json_from, Value};

impl BlockfrostAPI {
    pub async fn derive_address(
        &self, xpub: &str, role: &str, index: &str,
    ) -> BlockfrostResult<UtilsAddressesXpub> {
        self.call_endpoint(&format!(
            "/utils/addresses/xpub/{}/{}/{}",
            xpub, role, index
        ))
        .await
    }

    pub async fn utils_tx_evaluate(&self, transaction_data: Vec<u8>) -> BlockfrostResult<Value> {
        let body = Body::from(transaction_data);
        let url = Url::from_endpoint(self.base_url.as_str(), "/utils/txs/evaluate")?;

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
