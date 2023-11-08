use crate::{json_error, process_error_response, reqwest_error, BlockfrostError, RetrySettings};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde_json::from_str as json_from;
use std::{future::Future, thread};

// Used only for simple and common GET requests.
// Functions that require extra logic may not call this.
pub(crate) fn send_get_request<T>(
    client: &Client,
    url: String,
    retry_settings: RetrySettings,
) -> impl Future<Output = Result<T, BlockfrostError>> + Send
where
    T: serde::de::DeserializeOwned,
{
    let request = client.get(&url);

    async move {
        let (status, text) = send_request(request, retry_settings)
            .await
            .map_err(|reason| reqwest_error(&url, reason))?;

        if !status.is_success() {
            return Err(process_error_response(&text, status, &url));
        }

        json_from::<T>(&text).map_err(|reason| json_error(url, text, reason))
    }
}

// Send requests with delayed retries, cloning the request builder only when necessary.
pub(crate) async fn send_request_unprocessed(
    request: RequestBuilder,
    retry_settings: RetrySettings,
) -> reqwest::Result<Response> {
    for _ in 1..retry_settings.amount {
        let request = clone_request(&request);
        let response = request.send().await;

        if let Err(err) = &response {
            if let Some(StatusCode::TOO_MANY_REQUESTS) = err.status() {
                thread::sleep(retry_settings.delay);
                continue;
            }
        }

        return response;
    }
    request.send().await
}

// Calls send_request_unprocessed but break is down
pub(crate) async fn send_request(
    request: RequestBuilder,
    retry_settings: RetrySettings,
) -> reqwest::Result<(StatusCode, String)> {
    let response = send_request_unprocessed(request, retry_settings).await?;
    let status = response.status();
    let text = response.text().await?;

    Ok((status, text))
}

fn clone_request(request: &RequestBuilder) -> RequestBuilder {
    // Safety:
    //     Requests in this crate never use streams.
    //     .try_clone() will always succeed.
    request.try_clone().unwrap()
}
