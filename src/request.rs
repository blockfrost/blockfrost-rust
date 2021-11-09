//! Module for common requests logic.

use std::future::Future;

use reqwest::{Client, RequestBuilder, Response};

use crate::process_error_response;

// Used only for simple and common GET requests.
// Functions that require extra logic may not call this.
pub(crate) fn send_get_request<T>(
    client: &Client,
    url: String,
) -> impl Future<Output = crate::Result<T>> + Send
where
    T: serde::de::DeserializeOwned,
{
    let request = client.get(&url);

    async move {
        let response = send_request(request).await?;

        let status_code = response.status();
        let text = response.text().await?;

        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code, &url));
        }
        // This gon have to be removed
        Ok(serde_json::from_str::<T>(&text)?)
    }
}

pub async fn send_request(request: RequestBuilder) -> crate::Result<Response> {
    Ok(request.send().await?)
}
