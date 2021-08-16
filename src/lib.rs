//! Rust SDK for Blockfrost.io

pub const CARDANO_MAINNET_NETWORK: &str = "https://cardano-mainnet.blockfrost.io/api/v0";
pub const CARDANO_TESTNET_NETWORK: &str = "https://cardano-testnet.blockfrost.io/api/v0";

#[derive(Debug, Default)]
pub struct BlockFrostApi {
    public_id: String,
    settings: Settings,
}

impl BlockFrostApi {
    pub fn new(public_id: impl AsRef<str>, settings: Settings) -> Self {
        Self {
            public_id: public_id.as_ref().to_string(),
            settings,
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
