//! Rust SDK for Blockfrost.io

pub mod endpoints;
pub mod env;
pub mod error;
pub mod models;
pub mod settings;

const LIB_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub use error::{process_error, Error, HttpError, Result};
pub use settings::Settings;

use reqwest::{header::HeaderMap, Client};
use serde_json::from_str as serde_from_str;

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";

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
        let user_agent = format!("blockfrost-rust/{}", LIB_VERSION);

        headers.insert("project_id", project_id);
        headers.insert("User-Agent", user_agent.parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();
        Self { settings, client }
    }
}

// Private interface
impl BlockFrostApi {
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
            return Err(process_error(&text, status_code));
        }
        Ok(serde_from_str::<T>(&text)?)
    }

    fn gather_url(&self, suffix: &str) -> String {
        self.settings.network_endpoint.to_string() + suffix
    }
}
