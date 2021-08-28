//! Rust SDK for Blockfrost.io

pub mod env;
pub mod error;
pub mod models;
pub mod settings;

pub use error::{process_error, Error, HttpError, Result};

pub use settings::Settings;

use serde_json::from_str as serde_from_str;

use reqwest::{header::HeaderMap, Client};

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
        headers.insert("project_id", project_id);

        let client = Client::builder().default_headers(headers).build().unwrap();

        Self { settings, client }
    }

    pub async fn health(&self) -> Result<models::Health> {
        self.get("/health").await
    }

    pub async fn root(&self) -> Result<models::Root> {
        self.get("/").await
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

        if !status_code.is_success() {
            return Err(process_error(&text, status_code));
        }
        Ok(serde_from_str::<T>(&text)?)
    }

    fn gather_url(&self, suffix: &str) -> String {
        self.settings.network_endpoint.to_string() + suffix
    }
}
