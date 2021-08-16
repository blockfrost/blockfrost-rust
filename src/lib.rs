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

#[derive(Debug, Default)]
pub struct Settings;
