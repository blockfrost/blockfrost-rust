pub(crate) mod endpoints;
mod settings;

use std::future::Future;

use reqwest::{header::HeaderMap, Client};
use serde_json::from_str as serde_from_str;

use crate::error::process_error_response;
use crate::url::Url;
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

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    // Url endpoint example: "/blocks"
    pub(crate) fn get_from_endpoint<T>(&self, url_endpoint: &str) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
        let Url(url) = Url::from_endpoint(self.settings(), url_endpoint);
        Self::get_from_url(self, url)
    }

    // Url here is the full url
    pub(crate) fn get_from_url<T>(&self, url: String) -> impl Future<Output = crate::Result<T>> + Send
    where
        T: serde::de::DeserializeOwned,
    {
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
}
