pub(crate) mod endpoints;
mod settings;

use reqwest::{header::HeaderMap, Client};
use serde_json::from_str as serde_from_str;

use crate::error::process_error_response;
pub use settings::Settings;

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

    async fn get<T>(&self, url_suffix: &str) -> crate::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = self.gather_url(url_suffix);
        let response = self.client.get(url).send().await?;

        let status_code = response.status();
        let text = response.text().await?;

        let debug_info = format!("{}: {}", url_suffix, text);
        eprintln!("debug_info: {}.", debug_info);

        if !status_code.is_success() {
            return Err(process_error_response(&text, status_code));
        }
        // This gon have to be removed
        Ok(serde_from_str::<T>(&text)?)
    }

    fn gather_url(&self, suffix: &str) -> String {
        self.settings.network_endpoint.to_string() + suffix
    }
}
