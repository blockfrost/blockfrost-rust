//! Rust SDK for Blockfrost.io

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
}

#[derive(Debug)]
pub struct Settings {
    network_endpoint: &'static str,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            network_endpoint: CARDANO_MAINNET_NETWORK,
        }
    }

    pub fn set_test_network(&mut self, flag: bool) {
        self.network_endpoint = if flag {
            CARDANO_TESTNET_NETWORK
        } else {
            CARDANO_MAINNET_NETWORK
        };
    }

    pub fn set_custom_network(&mut self, network_endpoint: &'static str) {
        self.network_endpoint = network_endpoint;
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}
