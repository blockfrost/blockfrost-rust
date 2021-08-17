//! Rust SDK for Blockfrost.io

pub mod models;

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";

#[derive(Debug, Default)]
pub struct BlockFrostApi {
    project_id: String,
    settings: Settings,
    client: reqwest::Client,
}

impl BlockFrostApi {
    pub fn new(project_id: impl AsRef<str>, settings: Settings) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();

        headers.insert("project_id", project_id.as_ref().parse().unwrap());

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        Self {
            project_id: project_id.as_ref().to_string(),
            settings,
            client,
        }
    }

    pub async fn health(&self) -> reqwest::Result<models::Health> {
        let suffix = "/health";
        let full_url = self.settings.network_endpoint.to_string() + suffix;
        dbg!(&full_url);
        let response = self.client.get(full_url).send().await?;
        dbg!(&response);
        response.json().await
    }
}

#[derive(Debug)]
pub struct Settings {
    pub(crate) network_endpoint: &'static str,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            network_endpoint: CARDANO_MAINNET_NETWORK,
        }
    }

    pub fn set_test_network(mut self, flag: bool) -> Self {
        self.network_endpoint = if flag {
            CARDANO_TESTNET_NETWORK
        } else {
            CARDANO_MAINNET_NETWORK
        };
        self
    }

    pub fn set_custom_network(&mut self, network_endpoint: &'static str) {
        self.network_endpoint = network_endpoint;
    }

    pub fn current_network(&self) -> &'static str {
        self.network_endpoint
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
