pub(crate) mod endpoints;
mod settings;

use std::future::Future;

use reqwest::{header::HeaderMap, Client};
use serde_json::from_str as serde_from_str;

use crate::error::process_error_response;
pub use settings::*;

/// SDK version being used.
pub const USER_AGENT: &str = concat!("blockfrost-rust/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Clone)]
pub struct BlockFrostApi {
    settings: Settings,
    client: reqwest::Client,
}

// Public interface
impl BlockFrostApi {
    pub fn new(settings: Settings) -> Self {
        let mut headers = HeaderMap::new();

        let project_id = settings.project_id.parse().unwrap();

        headers.insert("project_id", project_id);
        headers.insert("User-Agent", USER_AGENT.parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { settings, client }
    }

    fn get<T>(&self, url_suffix: &str) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.gather_url(url_suffix);
        let response_future = self.client.get(&url).send();
        async move {
            let response = response_future.await?;

            let status_code = response.status();
            let text = response.text().await?;

            let debug_info = format!("{}: {}", url, text);
            eprintln!("debug_info: {}", debug_info);

            if !status_code.is_success() {
                return Err(process_error_response(&text, status_code));
            }
            // This gon have to be removed
            Ok(serde_from_str::<T>(&text)?)
        }
    }

    fn gather_url(&self, suffix: &str) -> String {
        let endpoint_url = self.settings.network_endpoint.to_string() + suffix;

        fn append_query_parameter(string: &mut String, parameter_name: &str, parameter: impl AsRef<str>) {
            if string.is_empty() {
                // Query parameters come after the question mark
                string.push('?');
            } else {
                // Separator between parameters
                string.push('&');
            }
            string.push_str(parameter_name);
            string.push('=');
            string.push_str(parameter.as_ref());
        }

        let QueryParameters { count, page, order, from, to } = &self.settings.query_parameters;
        let mut query_parameters = String::new();
        if let Some(count) = count {
            append_query_parameter(&mut query_parameters, "count", count.to_string());
        }
        if let Some(page) = page {
            append_query_parameter(&mut query_parameters, "page", page.to_string());
        }
        if let Some(order) = order {
            append_query_parameter(&mut query_parameters, "order", order.to_string());
        }
        if let Some(from) = from {
            append_query_parameter(&mut query_parameters, "from", from);
        }
        if let Some(to) = to {
            append_query_parameter(&mut query_parameters, "to", to);
        }

        // Finished url
        endpoint_url + &query_parameters
    }
}
