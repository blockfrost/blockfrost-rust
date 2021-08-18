use crate::{CARDANO_MAINNET_NETWORK, CARDANO_TESTNET_NETWORK};

#[derive(Debug, Clone)]
pub struct Settings {
    pub(crate) project_id: String,
    pub(crate) network_endpoint: &'static str,
}

impl Settings {
    pub fn new(project_id: impl AsRef<str>) -> Self {
        Self {
            network_endpoint: CARDANO_MAINNET_NETWORK,
            project_id: project_id.as_ref().to_string(),
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

    pub fn set_custom_network(&mut self, custom_network: &'static str) {
        self.network_endpoint = custom_network;
    }

    pub fn current_network(&self) -> &'static str {
        self.network_endpoint
    }
}
