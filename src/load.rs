/// Loader function to help configuring your API usage.
///
/// See [`configurations_from_env`].
use std::{
    env, fs,
    path::{Path, PathBuf},
};

use toml::Value as TomlValue;

/// Loads configuration from env vars and config file.
///
/// First searches for the configuration files `blockfrost.toml` and `.blockfrost.toml` in the
/// current and parent directories.
///
/// Scan starts at the current directory and goes up to the filesystem root, if found, the file is
/// loaded into a [`TomlValue`].
///
/// After that, load configs from ENV vars, possibly overwriting what was loaded before.
///
/// | `TOML` key        | Environment variable         |
/// |-------------------|------------------------------|
/// | `project_id`      | `BLOCKFROST_PROJECT_ID`      |
/// | `cardano_network` | `BLOCKFROST_CARDANO_NETWORK` |
/// | `ipfs_network`    | `BLOCKFROST_IPFS_NETWORK`    |
///
/// This means that if `BLOCKFROST_PROJECT_ID` is detected, you will be able to access it with
/// `toml_value["project_id"]`.
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
pub fn configurations_from_env() -> crate::Result<TomlValue> {
    let config_file = scan_first_config_file()?;

    let mut toml_value = match &config_file {
        Some(file_path) => load_toml_from_path(file_path)?,
        None => TomlValue::Table(Default::default()),
    };

    if let Ok(var) = env::var("BLOCKFROST_NETWORK_ADDRESS") {
        toml_value["blockfrost-network-address"] = toml::Value::String(var);
    }

    if let Ok(var) = env::var("BLOCKFROST_NETWORK_ADDRESS") {
        toml_value["blockfrost-project-id"] = toml::Value::String(var);
    }

    Ok(toml_value)
}

fn load_toml_from_path(path: &Path) -> crate::Result<TomlValue> {
    Ok(toml::from_str(&fs::read_to_string(path)?)?)
}

// Scans for the first 'blockfrost.toml' or '.blockfrost.toml' file in the current or parent
// directories. Scan goes up to the root of the filesystem.
fn scan_first_config_file() -> crate::Result<Option<PathBuf>> {
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
