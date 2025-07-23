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
    for _ in 1..retry_settings.amount {
        let request = clone_request(&request);
        let response = request.send().await;
        let retry_codes = [
            StatusCode::REQUEST_TIMEOUT,
            StatusCode::PAYLOAD_TOO_LARGE,
            StatusCode::TOO_MANY_REQUESTS,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
            StatusCode::SERVICE_UNAVAILABLE,
            StatusCode::GATEWAY_TIMEOUT,
        ];

        if let Err(err) = &response {
            if let Some(status) = err.status() {
                if retry_codes.contains(&status) {
                    Delay::new(retry_settings.delay).await;
                    continue;
                }
            }

            return response;
        } else {
            return response;
        }
    }

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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::pagination::{Order, Pagination};
//     use httpmock::{Method::GET, Mock, MockServer};
//     use reqwest::Client;

//     // helper to set up test environment
//     fn setup_test() -> (MockServer, Client, RetrySettings, String) {
//         let server = MockServer::start();
//         let client = Client::new();
//         let retry_settings = RetrySettings {
//             amount: 2,
//             delay: std::time::Duration::from_millis(100),
//         };
//         let base_url = server.url("/items");

//         (server, client, retry_settings, base_url)
//     }

//     // helper to set up page mocks
//     fn setup_page_mock<'a>(server: &'a MockServer, page: u32, status: u16, body: &str) -> Mock<'a> {
//         server.mock(|when, then| {
//             when.method(GET)
//                 .path("/items")
//                 .query_param("page", page.to_string());
//             then.status(status)
//                 .header("Content-Type", "application/json")
//                 .body(body);
//         })
//     }

//     #[tokio::test]
//     async fn test_fetch_all_pages_success_multi_page() {
//         let (server, client, retry_settings, base_url) = setup_test();

//         // mocks
//         setup_page_mock(&server, 1, 200, "[1, 2]");
//         setup_page_mock(&server, 2, 200, "[3, 4]");
//         setup_page_mock(&server, 3, 200, "[]");

//         let pagination = Pagination::all();
//         let batch_size = 1;

//         let result =
//             fetch_all_pages::<u32>(&client, &base_url, retry_settings, pagination, batch_size)
//                 .await
//                 .unwrap();

//         assert_eq!(vec![1, 2, 3, 4], result);
//     }

//     #[tokio::test]
//     async fn test_fetch_all_pages_exact_count() {
//         let (server, client, retry_settings, base_url) = setup_test();

//         // mocks
//         setup_page_mock(&server, 1, 200, "[1, 2]");
//         setup_page_mock(&server, 2, 200, "[3]");

//         let pagination = Pagination {
//             page: 1,
//             count: 3,
//             order: Order::Asc,
//             fetch_all: false,
//         };
//         let batch_size = 1;

//         let result =
//             fetch_all_pages::<u32>(&client, &base_url, retry_settings, pagination, batch_size)
//                 .await
//                 .unwrap();

//         assert_eq!(vec![1, 2, 3], result);
//     }

//     #[tokio::test]
//     async fn test_fetch_all_pages_empty_first_page() {
//         let (server, client, retry_settings, base_url) = setup_test();

//         // mocks
//         setup_page_mock(&server, 1, 200, "[]");

//         let pagination = Pagination {
//             page: 1,
//             count: 100,
//             order: Order::Asc,
//             fetch_all: true,
//         };
//         let batch_size = 1;

//         let result =
//             fetch_all_pages::<u32>(&client, &base_url, retry_settings, pagination, batch_size)
//                 .await
//                 .unwrap();

//         assert_eq!(Vec::<u32>::new(), result);
//     }
// }
