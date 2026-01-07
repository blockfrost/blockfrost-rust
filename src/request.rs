use crate::{
    json_error, pagination::Pagination, process_error_response, reqwest_error, url::Url,
    BlockfrostError, RetrySettings,
};
use futures::future::try_join_all;
use futures_timer::Delay;
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::from_str;
use std::future::Future;

// Used only for simple and common GET requests.
// Functions that require extra logic may not call this.
pub(crate) fn send_get_request<T>(
    client: &Client, url: String, retry_settings: RetrySettings,
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
    request: RequestBuilder, retry_settings: RetrySettings,
) -> reqwest::Result<Response> {
    let retry_codes = [
        StatusCode::REQUEST_TIMEOUT,
        StatusCode::PAYLOAD_TOO_LARGE,
        StatusCode::TOO_MANY_REQUESTS,
        StatusCode::INTERNAL_SERVER_ERROR,
        StatusCode::BAD_GATEWAY,
        StatusCode::SERVICE_UNAVAILABLE,
        StatusCode::GATEWAY_TIMEOUT,
    ];

    // Try all attempts except the last one (cloning the request each time)
    for _ in 1..retry_settings.amount {
        let response = clone_request(&request).send().await;

        let should_retry = match &response {
            Ok(resp) => retry_codes.contains(&resp.status()),
            Err(err) => err.status().is_some_and(|s| retry_codes.contains(&s)),
        };

        if should_retry {
            Delay::new(retry_settings.delay).await;
        } else {
            return response;
        }
    }

    // final attempt
    request.send().await
}

// Calls send_request_unprocessed but break is down
pub(crate) async fn send_request(
    request: RequestBuilder, retry_settings: RetrySettings,
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

async fn fetch_page<T: DeserializeOwned>(
    client: Client, url: String, retry: RetrySettings,
) -> Result<Vec<T>, BlockfrostError> {
    let request = client.get(&url);
    let (status, text) = send_request(request, retry)
        .await
        .map_err(|e| reqwest_error(&url, e))?;

    if !status.is_success() {
        return Err(process_error_response(&text, status, &url));
    }

    from_str::<Vec<T>>(&text).map_err(|e| json_error(url, text, e))
}

pub(crate) async fn fetch_all_pages<T: DeserializeOwned>(
    client: &Client, base_url: &str, retry: RetrySettings, pagination: Pagination,
    batch_size: usize,
) -> Result<Vec<T>, BlockfrostError> {
    let mut all = Vec::new();
    let mut page_start = pagination.page;
    let mut last_size = 0;

    loop {
        let urls = Url::generate_batch(base_url, batch_size, page_start, pagination)?;
        let client_cloned = client.clone();

        let pages: Vec<Vec<T>> = try_join_all(
            urls.into_iter()
                .map(|u| fetch_page::<T>(client_cloned.clone(), u, retry)),
        )
        .await?;

        let mut saw_any = false;

        for mut page in pages {
            if !page.is_empty() {
                saw_any = true;
            }

            all.append(&mut page);
        }

        if !saw_any || all.len() == last_size {
            break;
        }

        last_size = all.len();

        page_start += batch_size;
    }

    Ok(all)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pagination::Pagination;
    use httpmock::{Method::GET, Mock, MockServer};
    use reqwest::Client;

    fn setup_test() -> (MockServer, Client, RetrySettings, String) {
        let server = MockServer::start();
        let client = Client::new();
        let retry_settings = RetrySettings {
            amount: 2,
            delay: std::time::Duration::from_millis(100),
        };
        let base_url = server.url("/items");

        (server, client, retry_settings, base_url)
    }

    fn setup_page_mock<'a>(server: &'a MockServer, page: u32, status: u16, body: &str) -> Mock<'a> {
        server.mock(|when, then| {
            when.method(GET)
                .path("/items")
                .query_param("page", page.to_string());
            then.status(status)
                .header("Content-Type", "application/json")
                .body(body);
        })
    }

    #[tokio::test]
    async fn test_fetch_all_pages_success_multi_page() {
        let (server, client, retry_settings, base_url) = setup_test();

        // mocks
        setup_page_mock(&server, 1, 200, "[1, 2]");
        setup_page_mock(&server, 2, 200, "[3, 4]");
        setup_page_mock(&server, 3, 200, "[]");

        let pagination = Pagination::all();
        let batch_size = 1;

        let result =
            fetch_all_pages::<u32>(&client, &base_url, retry_settings, pagination, batch_size)
                .await
                .unwrap();

        assert_eq!(vec![1, 2, 3, 4], result);
    }

    #[tokio::test]
    async fn test_fetch_all_pages_success_multi_page_different_batch_size() {
        let (server, client, retry_settings, base_url) = setup_test();

        // mocks
        setup_page_mock(&server, 1, 200, "[1, 2]");
        setup_page_mock(&server, 2, 200, "[3, 4]");
        setup_page_mock(&server, 3, 200, "[5, 6]");
        setup_page_mock(&server, 4, 200, "[7, 8]");
        setup_page_mock(&server, 5, 200, "[9]");
        setup_page_mock(&server, 6, 200, "[]");
        setup_page_mock(&server, 7, 200, "[]");
        setup_page_mock(&server, 8, 200, "[]");
        setup_page_mock(&server, 9, 200, "[]");
        setup_page_mock(&server, 10, 200, "[]");

        let pagination = Pagination::all();
        let batch_size = 5;

        let result =
            fetch_all_pages::<u32>(&client, &base_url, retry_settings, pagination, batch_size)
                .await
                .unwrap();

        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], result);
    }

    #[tokio::test]
    async fn test_success_returns_immediately() {
        let server = MockServer::start();
        let client = Client::new();
        let retry_settings = RetrySettings {
            amount: 3,
            delay: std::time::Duration::from_millis(10),
        };

        let mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(200).body("ok");
        });

        let request = client.get(server.url("/test"));
        let response = send_request_unprocessed(request, retry_settings)
            .await
            .unwrap();

        // 200 should hit only once - no retry for success
        assert_eq!(response.status(), 200);
        assert_eq!(mock.calls(), 1);
    }

    #[tokio::test]
    async fn test_no_retry_on_client_error() {
        let server = MockServer::start();
        let client = Client::new();
        let retry_settings = RetrySettings {
            amount: 3,
            delay: std::time::Duration::from_millis(10),
        };

        let mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(404).body("not found");
        });

        let request = client.get(server.url("/test"));
        let response = send_request_unprocessed(request, retry_settings)
            .await
            .unwrap();

        // 404 should hit only once - no retry for success
        assert_eq!(response.status(), 404);
        assert_eq!(mock.calls(), 1);
    }

    #[tokio::test]
    async fn test_retry_on_server_error() {
        let server = MockServer::start();
        let client = Client::new();
        let retry_settings = RetrySettings {
            amount: 3,
            delay: std::time::Duration::from_millis(10),
        };

        let mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(503);
        });

        let request = client.get(server.url("/test"));
        let response = send_request_unprocessed(request, retry_settings)
            .await
            .unwrap();

        // 503 is retryable, should hit exactly `amount` times
        assert_eq!(response.status(), 503);
        assert_eq!(mock.calls(), 3);
    }

    #[tokio::test]
    async fn test_single_attempt_no_retry() {
        let server = MockServer::start();
        let client = Client::new();
        let retry_settings = RetrySettings {
            amount: 1,
            delay: std::time::Duration::from_millis(10),
        };

        let mock = server.mock(|when, then| {
            when.method(GET).path("/test");
            then.status(503);
        });

        let request = client.get(server.url("/test"));
        let response = send_request_unprocessed(request, retry_settings)
            .await
            .unwrap();

        assert_eq!(response.status(), 503);
        assert_eq!(mock.calls(), 1);
    }
}
