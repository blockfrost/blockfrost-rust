//! Module for common requests logic.

use std::{future::Future, thread};

use reqwest::{Client, RequestBuilder, Response, StatusCode};

use crate::{process_error_response, RetrySettings};

// Used only for simple and common GET requests.
// Functions that require extra logic may not call this.
pub(crate) fn send_get_request<T>(
    client: &Client,
    url: String,
    retry_settings: RetrySettings,
) -> impl Future<Output = crate::Result<T>> + Send
where
    T: serde::de::DeserializeOwned,
{
    let request = client.get(&url);

    async move {
        let (status_code, text) = send_request(request, retry_settings).await?;

        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code, &url));
        }
        // This gon have to be removed
        Ok(serde_json::from_str::<T>(&text)?)
    }
}

// Send requests with delayed retries, cloning the request builder only when necessary.
pub(crate) async fn send_request_unprocessed(
    request: RequestBuilder,
    retry_settings: RetrySettings,
) -> crate::Result<Response> {
    for _ in 1..retry_settings.amount {
        let request = clone_request(&request);
        let response = request.send().await;

        if let Err(err) = &response {
            if let Some(StatusCode::TOO_MANY_REQUESTS) = err.status() {
                thread::sleep(retry_settings.delay);
                continue;
            }
        }

        return Ok(response?);
    }
    Ok(request.send().await?)
}

// Calls send_request_unprocessed but break is down
pub(crate) async fn send_request(
    request: RequestBuilder,
    retry_settings: RetrySettings,
) -> crate::Result<(StatusCode, String)> {
    let response = send_request_unprocessed(request, retry_settings).await?;
    let status_code = response.status();
    let text = response.text().await?;

    Ok((status_code, text))
}

fn clone_request(request: &RequestBuilder) -> RequestBuilder {
    // Safety:
    //     Requests in this crate never use streams.
    //     .try_clone() will always succeed.
    request.try_clone().unwrap()
}
