use crate::{
    json_error, process_error_response, reqwest_error, url::Url, BlockfrostError, Pagination,
    RetrySettings,
};
use futures::future;
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::from_str;
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

        from_str::<T>(&text).map_err(|reason| json_error(url, text, reason))
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

pub(crate) async fn fetch_all_pages<T: DeserializeOwned>(
    client: &Client,
    url: String,
    retry_settings: RetrySettings,
    pagination: Pagination,
) -> Result<Vec<T>, BlockfrostError> {
    const BATCH_SIZE: usize = 10;

    let mut page_start: usize = 1;
    let mut is_end = false;
    let mut result = Vec::new();

    while !is_end {
        let batch = Url::generate_batch(url.as_str(), BATCH_SIZE, page_start, pagination)?;
        let bodies: Vec<Result<Vec<T>, BlockfrostError>> =
            future::join_all(batch.into_iter().map(|url| {
                let client = client.clone();
                async move {
                    let request = client.get(&url);
                    let (status, text) = send_request(request, retry_settings)
                        .await
                        .map_err(|reason| reqwest_error(&url, reason))?;

                    if !status.is_success() {
                        return Err(process_error_response(&text, status, &url));
                    }

                    from_str::<Vec<T>>(&text).map_err(|reason| json_error(url, text, reason))
                }
            }))
            .await;

        for b in bodies {
            match b {
                Ok(data) => {
                    if data.len() < pagination.count {
                        is_end = true;
                    }
                    result.extend(data);
                },
                Err(err) => return Err(err),
            }
        }

        page_start += BATCH_SIZE;
    }

    Ok(result)
}
