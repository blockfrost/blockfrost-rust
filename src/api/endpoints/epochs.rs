use crate::*;
use serde::{Deserialize, Serialize};

impl BlockFrostApi {
    endpoints! {
        /// Return the information about the latest, therefore current, epoch.
        epochs_latest() -> Epoch => "/epochs/latest";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1latest/get"),

        /// Return the protocol parameters for the latest epoch.
        epochs_latest_parameters() -> EpochParameters => "/epochs/latest/parameters";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1latest~1parameters/get"),

        /// Return the content of the requested epoch.
        epochs_by_number(number: Integer) -> Epoch => "/epochs/{number}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}/get"),

        /// Return the protocol parameters for the epoch specified
        epochs_parameters(number: Integer) -> EpochParameters => "/epochs/{number}/parameters";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1parameters/get"),
    }
    paged_endpoints! {
        /// Return the list of epochs following a specific epoch.
        epochs_next(number: Integer) -> Vec<Epoch> => "/epochs/{number}/next";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1next/get"),

        /// Return the list of epochs preceding a specific epoch.
        epochs_previous(number: Integer) -> Vec<Epoch> => "/epochs/{number}/previous";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1previous/get"),

        /// Return the active stake distribution for the specified epoch.
        epochs_stakes(number: Integer) -> Vec<AddressStakePool> => "/epochs/{number}/stakes";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1stakes/get"),

        /// Return the active stake distribution for the epoch specified by stake pool.
        epochs_stakes_by_pool(number: Integer, pool_id: &str) -> Vec<AddressStake> => "/epochs/{number}/stakes/{pool_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1stakes~1{pool_id}/get"),

        /// Return the blocks minted for the epoch specified.
        epochs_blocks(number: Integer) -> Vec<String> => "/epochs/{number}/blocks";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1blocks/get"),

        /// Return the block minted for the epoch specified by stake pool.
        epochs_blocks_by_pool(number: Integer, pool_id: &str) -> Vec<String> => "/epochs/{number}/blocks/{pool_id}";
            ("https://docs.blockfrost.io/#tag/Cardano-Epochs/paths/~1epochs~1{number}~1blocks~1{pool_id}/get"),
    }
}

/// Created by [`epochs_latest`](BlockFrostApi::epochs_latest) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Epoch {
    /// Epoch number.
    pub epoch: Integer,
    /// Unix time of the start of the epoch.
    pub start_time: Integer,
    /// Unix time of the end of the epoch.
    pub end_time: Integer,
    /// Unix time of the first block of the epoch.
    pub first_block_time: Integer,
    /// Unix time of the last block of the epoch.
    pub last_block_time: Integer,
    /// Number of blocks within the epoch.
    pub block_count: Integer,
    /// Number of transactions within the epoch.
    pub tx_count: Integer,
    /// Sum of all the transactions within the epoch in Lovelaces.
    pub output: String,
    /// Sum of all the fees within the epoch in Lovelaces.
    pub fees: String,
    /// Sum of all the active stakes within the epoch in Lovelaces.
    pub active_stake: Option<String>,
}

/// Created by [`epochs_latest_parameters`](BlockFrostApi::epochs_latest_parameters) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EpochParameters {
    /// Epoch number
    pub epoch: Integer,
    /// The linear factor for the minimum fee calculation for a given epoch.
    pub min_fee_a: Integer,
    /// The constant factor for the minimum fee calculation.
    pub min_fee_b: Integer,
    /// Maximum block body size in Bytes.
    pub max_block_size: Integer,
    /// Maximum transaction size.
    pub max_tx_size: Integer,
    /// Maximum block header size.
    pub max_block_header_size: Integer,
    /// The amount of a key registration deposit in Lovelaces.
    pub key_deposit: String,
    /// The amount of a pool registration deposit in Lovelaces.
    pub pool_deposit: String,
    // Epoch bound on pool retirement.
    pub e_max: Integer,
    /// Desired number of pools.
    pub n_opt: Integer,
    /// Pool pledge influence.
    pub a0: Float,
    /// Monetary expansion.
    pub rho: Float,
    /// Treasury expansion.
    pub tau: Float,
    /// Percentage of blocks produced by federated nodes
    pub decentralisation_param: Float,
    /// Seed for extra entropy.
    pub extra_entropy: Option<JsonMap>,
    /// Accepted protocol major version.
    pub protocol_major_ver: Integer,
    /// Accepted protocol minor version.
    pub protocol_minor_ver: Integer,
    /// Minimum UTXO value.
    pub min_utxo: String,
    /// Minimum stake cost forced on the pool.
    pub min_pool_cost: String,
    /// Epoch number only used once.
    pub nonce: String,
    /// The per word cost of script memory usage.
    pub price_mem: Option<Float>,
    /// The cost of script execution step usage.
    pub price_step: Option<Float>,
    /// The maximum number of execution memory allowed to be used in a single transaction.
    pub max_tx_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single transaction.
    pub max_tx_ex_steps: Option<String>,
    /// The maximum number of execution memory allowed to be used in a single block.
    pub max_block_ex_mem: Option<String>,
    /// The maximum number of execution steps allowed to be used in a single block.
    pub max_block_ex_steps: Option<String>,
    /// The maximum Val size.
    pub max_val_size: Option<String>,
    /// The percentage of the transactions fee which must be provided as collateral when including
    /// non-native scripts.
    pub collateral_percent: Option<Float>,
    /// The maximum number of collateral inputs allowed in a transaction.
    pub max_collateral_inputs: Option<Integer>,
    /// The cost per UTxO word.
    pub coins_per_utxo_word: Option<String>,
}

/// Created by [`epochs_stakes`](BlockFrostApi::epochs_stakes) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressStakePool {
    /// Stake address.
    pub stake_address: String,
    /// Bech32 prefix of the pool delegated to.
    pub pool_id: String,
    /// Amount of active delegated stake in Lovelaces.
    pub amount: String,
}

/// Created by [`epochs_stakes_by_pool`](BlockFrostApi::epochs_stakes_by_pool) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AddressStake {
    /// Stake address.
    pub stake_address: String,
    /// Amount of active delegated stake in Lovelaces.
    pub amount: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_example! { test_epochs_latest, Epoch, r#"
    {
        "epoch": 225,
        "start_time": 1603403091,
        "end_time": 1603835086,
        "first_block_time": 1603403092,
        "last_block_time": 1603835084,
        "block_count": 21298,
        "tx_count": 17856,
        "output": "7849943934049314",
        "fees": "4203312194",
        "active_stake": "784953934049314"
    }
    "# }

    test_example! { test_epochs_latest_parameters, EpochParameters, r#"
    {
        "epoch": 225,
        "min_fee_a": 44,
        "min_fee_b": 155381,
        "max_block_size": 65536,
        "max_tx_size": 16384,
        "max_block_header_size": 1100,
        "key_deposit": "2000000",
        "pool_deposit": "500000000",
        "e_max": 18,
        "n_opt": 150,
        "a0": 0.3,
        "rho": 0.003,
        "tau": 0.2,
        "decentralisation_param": 0.5,
        "extra_entropy": null,
        "protocol_major_ver": 2,
        "protocol_minor_ver": 0,
        "min_utxo": "1000000",
        "min_pool_cost": "340000000",
        "nonce": "1a3be38bcbb7911969283716ad7aa550250226b76a61fc51cc9a9a35d9276d81",
        "price_mem": 0.001,
        "price_step": 0.01,
        "max_tx_ex_mem": "11000000000",
        "max_tx_ex_steps": "11000000000",
        "max_block_ex_mem": "110000000000",
        "max_block_ex_steps": "110000000000",
        "max_val_size": "5000",
        "collateral_percent": 1.5,
        "max_collateral_inputs": 6,
        "coins_per_utxo_word": "34482"
    }
    "# }

    test_example! { test_epochs_next, Vec<Epoch>, r#"
    [
        {
            "epoch": 225,
            "start_time": 1603403091,
            "end_time": 1603835086,
            "first_block_time": 1603403092,
            "last_block_time": 1603835084,
            "block_count": 21298,
            "tx_count": 17856,
            "output": "7849943934049314",
            "fees": "4203312194",
            "active_stake": "784953934049314"
        }
    ]"#}

    test_example! { test_epochs_stakes, Vec<AddressStakePool>, r#"
    [
        {
            "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
            "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
            "amount": "4440295078"
        }
    ]
    "#}

    test_example! { test_epochs_stakes_by_pool, Vec<AddressStake>, r#"
    [
        {
            "stake_address": "stake1u9l5q5jwgelgagzyt6nuaasefgmn8pd25c8e9qpeprq0tdcp0e3uk",
            "amount": "4440295078"
        }
    ]
    "#}

    test_example! { test_epochs_blocks, Vec<String>, r#"
    [
        "d0fa315687e99ccdc96b14cc2ea74a767405d64427b648c470731a9b69e4606e",
        "38bc6efb92a830a0ed22a64f979d120d26483fd3c811f6622a8c62175f530878",
        "f3258fcd8b975c061b4fcdcfcbb438807134d6961ec278c200151274893b6b7d"
    ]
    "#}
}
