//! Utils for loading common settings from config file and environment variables.
//!
//! See [`configurations_from_env`].

use std::{
    env, fs,
    path::{Path, PathBuf},
};

use toml::{value::Map, Value as TomlValue};

use crate::Error;

const NETWORKS: [&str; 2] = ["cardano", "ipfs"];

/// Loads configuration from env vars and config file.
///
/// Searches for the files `blockfrost.toml` and `.blockfrost.toml` in the current, if not found,
/// scans each parent directory up to the filesystem root, if found, the file is loaded into a
/// [`toml::Value`].
///
/// After that, loads configs from ENV vars, possibly overwriting configurations loaded from config
/// file.
///
/// Note: if no config file is found, this function only loads configurations from ENV vars, and
/// it's guaranteed to not panic or return errors.
///
/// If you want to use both IPFS and Cardano, you can specify a table in the TOML file or use
/// environment variables to configure both project ids.
///
/// | `TOML` key           | Environment variable            |
/// |----------------------|---------------------------------|
/// | `project_id`         | `BLOCKFROST_PROJECT_ID`         |
/// | `cardano_network`    | `BLOCKFROST_CARDANO_NETWORK`    |
/// | `ipfs_network`       | `BLOCKFROST_IPFS_NETWORK`       |
/// | `cardano.project_id` | `BLOCKFROST_CARDANO_PROJECT_ID` |
/// | `ipfs.project_id`    | `BLOCKFROST_IPFS_PROJECT_ID`    |
///
/// This means that if `BLOCKFROST_PROJECT_ID` is detected, you will be able to access it with
/// `toml_value["project_id"]`. If `BLOCKFROST_IPFS_PROJECT_ID` is configured, you could access
/// it with `toml_value["ipfs"]["project_id"].
///
/// # TOML file:
///
/// Here's an example on how to TOML file could look like:
///
/// ```toml
/// project_id = "RXVW6SzwSojl2IXpKucPQBB7QgQoMTTe"
/// cardano_network = "https://cardano-mainnet.blockfrost.io/api/v0"
/// ipfs_network = "https://ipfs.blockfrost.io/api/v0"
/// ```
///
/// If you want to use both IPFS and Cardano, your TOML file could look like this:
/// ```toml
/// [cardano]
/// project_id = "mainnetXXXX"
/// network = "https://cardano-mainnet.blockfrost.io/api/v0"
///
/// [ipfs]
/// project_id = "ipfsXXXX"
/// network = "https://ipfs.blockfrost.io/api/v0"
/// ```
///
/// # Loading configs example:
///
/// Here's how this function can be used to build a [`BlockFrostApi`](crate::BlockFrostApi).
///
/// ```
/// use blockfrost::{load, BlockFrostApi};
///
/// fn build_api() -> blockfrost::Result<BlockFrostApi> {
///     let configurations = load::configurations_from_env()?;
///     let project_id = configurations["project_id"].as_str().unwrap();
///     let api = BlockFrostApi::new(project_id, Default::default());
///     Ok(api)
/// }
/// ```
///
/// Or to build specific IPFS and Cardano clients:
///
/// ```
/// use blockfrost::{load, BlockFrostApi, BlockFrostSettings, IpfsApi, IpfsSettings};
///
/// fn build_cardano_api() -> Option<BlockFrostApi> {
///     let config = load::configurations_from_env().ok()?;
///     let cardano_config = &config["cardano"];
///     match (cardano_config["project_id"].as_str(), cardano_config["network"].as_str()) {
///         (Ok(project_id), Ok(network)) => {
///             let api = BlockFrostApi::new(project_id, BlockFrostSettings {
///                 network_address: network.to_string(),
///                 ..Default::default(),
///             });
///
///             Some(api)
///         }
///         _ => None
///     }
/// }
///
/// fn build_blockfrost_ipfs() -> Option<IpfsApi> {
///    let config = blockfrost::load::configurations_from_env().ok()?;
///    let ipfs_config = &config["ipfs"];
///    match (ipfs_config["project_id"].as_str(), ipfs_config["network"].as_str()) {
///        (Some(project_id), Some(network)) => {
///            let api = IpfsApi::new(project_id, IpfsSettings {
///                network_address: network.to_string(),
///                ..Default::default()
///            });
///
///            Some(api)
///        }
///        _ => None,
///    }
///}
/// ```
pub fn configurations_from_env() -> crate::Result<TomlValue> {
    let config_file = scan_directories_for_config_file()?;

    let toml_value = match &config_file {
        Some(file_path) => load_toml_from_path(file_path)?,
        None => TomlValue::Table(Default::default()),
    };

    Ok(toml_value.as_table().map_or(toml_value.clone(), |x| {
        let mut toml_table = x.to_owned();

        if let Ok(var) = env::var("BLOCKFROST_PROJECT_ID") {
            toml_table.insert("project_id".to_string(), TomlValue::String(var));
        }

        if let Ok(var) = env::var("BLOCKFROST_CARDANO_NETWORK") {
            toml_table.insert("cardano_network".to_string(), TomlValue::String(var));
        }

        if let Ok(var) = env::var("BLOCKFROST_IPFS_NETWORK") {
            toml_table.insert("ipfs_network".to_string(), TomlValue::String(var));
        }

        for network in NETWORKS {
            build_network_config(network).map(|x| {
                toml_table.insert(network.to_string(), x);
            });
        }

        TomlValue::Table(toml_table)
    }))
}

fn load_toml_from_path(path: &Path) -> crate::Result<TomlValue> {
    let path = path.to_owned();
    let text = fs::read_to_string(&path)?;
    toml::from_str(&text).map_err(|reason| Error::Toml { reason, path })
}

// Scans for the first 'blockfrost.toml' or '.blockfrost.toml' file in the current or parent
// directories. Scan goes up to the root of the filesystem.
fn scan_directories_for_config_file() -> crate::Result<Option<PathBuf>> {
    let current_dir = env::current_dir()?;
    let mut current_dir = current_dir.as_path();

    loop {
        let candidate = current_dir.join("blockfrost.toml");
        if candidate.exists() {
            return Ok(Some(candidate));
        }

        let candidate = current_dir.join(".blockfrost.toml");
        if candidate.exists() {
            return Ok(Some(candidate));
        }

        if let Some(parent) = current_dir.parent() {
            current_dir = parent
        } else {
            return Ok(None);
        }
    }
}

// Builds network specific configuration from environment
fn build_network_config(name: &str) -> Option<TomlValue> {
    match env::var(format!("BLOCKFROST_{}_PROJECT_ID", name.to_uppercase())) {
        Ok(var) => {
            let mut map = Map::from_iter([("project_id".to_string(), TomlValue::String(var))]);

            if let Ok(var) = env::var(format!("BLOCKFROST_{}_NETWORK", name.to_uppercase())) {
                map.insert("network".to_string(), TomlValue::String(var));
            }

            Some(TomlValue::Table(map))
        }
        _ => None,
    }
}
