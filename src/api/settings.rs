use crate::{CARDANO_MAINNET_NETWORK, CARDANO_TESTNET_NETWORK};

#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) project_id: String,
    pub(crate) network_endpoint: String,
}

impl Settings {
    pub fn new(project_id: impl AsRef<str>) -> Self {
        Self { network_endpoint: CARDANO_MAINNET_NETWORK.to_string(), project_id: project_id.as_ref().to_string() }
    }

    pub fn set_test_network(mut self, flag: bool) -> Self {
        let address = if flag { CARDANO_TESTNET_NETWORK } else { CARDANO_MAINNET_NETWORK };
        self.network_endpoint = address.to_owned();
        self
    }

    pub fn set_network_address(&mut self, custom_network: impl AsRef<str>) {
        self.network_endpoint = custom_network.as_ref().to_string();
    }

    pub fn current_network(&self) -> &str {
        &self.network_endpoint
    }
}
