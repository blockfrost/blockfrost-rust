use serde::{Deserialize, Serialize};

use crate::*;

impl BlockFrostApi {
    /// Information about a specific stake account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}`].
    ///
    /// [`/accounts/{stake_address}`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}/get
    pub async fn accounts(&self, stake_address: &str) -> Result<Account> {
        let suffix = format!("/accounts/{stake_address}", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Reward history of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/rewards`].
    ///
    /// [`/accounts/{stake_address}/rewards`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1rewards/get
    pub async fn accounts_rewards(&self, stake_address: &str) -> Result<Vec<AccountReward>> {
        let suffix = format!("/accounts/{stake_address}/rewards", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// History of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/history`].
    ///
    /// [`/accounts/{stake_address}/history`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1history/get
    pub async fn accounts_history(&self, stake_address: &str) -> Result<Vec<AccountHistory>> {
        let suffix = format!("/accounts/{stake_address}/history", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Delegation information of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/delegations`].
    ///
    /// [`/accounts/{stake_address}/delegations`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1delegationsget
    pub async fn accounts_delegations(&self, stake_address: &str) -> Result<Vec<AccountDelegation>> {
        let suffix = format!("/accounts/{stake_address}/delegations", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// History of registrations and deregistrations of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/registrations`].
    ///
    /// [`/accounts/{stake_address}/registrations`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1registrationsget
    pub async fn accounts_registrations(&self, stake_address: &str) -> Result<Vec<AccountRegistration>> {
        let suffix = format!("/accounts/{stake_address}/registrations", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Withdrawal history of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/withdrawals`].
    ///
    /// [`/accounts/{stake_address}/withdrawals`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1withdrawals/get
    pub async fn accounts_withdrawals(&self, stake_address: &str) -> Result<Vec<AccountWithdrawal>> {
        let suffix = format!("/accounts/{stake_address}/withdrawals", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Obtain information about the MIRs of a specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/mirs`].
    ///
    /// [`/accounts/{stake_address}/mirs`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1mirs/get
    pub async fn accounts_mirs(&self, stake_address: &str) -> Result<Vec<AccountMir>> {
        let suffix = format!("/accounts/{stake_address}/mirs", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Addresses associated with specific account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/addresses`].
    ///
    /// [`/accounts/{stake_address}/addresses`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1addresses/get
    pub async fn accounts_addresses(&self, stake_address: &str) -> Result<Vec<AccountAddress>> {
        let suffix = format!("/accounts/{stake_address}/addresses", stake_address = stake_address);
        self.get(&suffix).await
    }

    /// Assets associated with specific account.
    ///
    /// **Be careful**, as an account could be part of a mangled address and does not necessarily
    /// mean the addresses are owned by user as the account.
    ///
    /// OpenAPI endpoint reference: [`/accounts/{stake_address}/addresses/assets`].
    ///
    /// [`/accounts/{stake_address}/addresses/assets`]: https://docs.blockfrost.io/#tag/Cardano-Accounts/paths/~1accounts~1{stake_address}~1addresses~1assets/get
    pub async fn accounts_addresses_assets(&self, stake_address: &str) -> Result<Vec<AccountAddressAsset>> {
        let suffix = format!("/accounts/{stake_address}/addresses/assets", stake_address = stake_address);
        self.get(&suffix).await
    }
}

/// Created by [`accounts`](BlockFrostApi::accounts) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    /// Bech32 stake address.
    pub stake_address: String,
    /// Registration state of an account.
    pub active: bool,
    /// Epoch of the most recent action - registration or deregistration.
    pub active_epoch: Integer,
    /// Balance of the account in Lovelaces.
    pub controlled_amount: String,
    /// Sum of all rewards for the account in the Lovelaces.
    pub rewards_sum: String,
    /// Sum of all the withdrawals for the account in Lovelaces.
    pub withdrawals_sum: String,
    /// Sum of all  funds from reserves for the account in the Lovelaces.
    pub reserves_sum: String,
    /// Sum of all funds from treasury for the account in the Lovelaces.
    pub treasury_sum: String,
    /// Sum of available rewards that haven't been withdrawn yet for the account in the Lovelaces.
    pub withdrawable_amount: String,
    /// Bech32 pool ID that owns the account.
    pub pool_id: Option<String>,
}

/// Created by [`accounts_rewards`](BlockFrostApi::accounts_rewards) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountReward {
    /// Epoch of the associated reward.
    pub epoch: Integer,
    /// Rewards for given epoch in Lovelaces.
    pub amount: String,
    /// Bech32 pool ID being delegated to.
    pub pool_id: String,
}

/// Created by [`accounts_history`](BlockFrostApi::accounts_history) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountHistory {
    /// Epoch of the associated reward.
    pub active_epoch: Integer,
    /// Rewards for given epoch in Lovelaces.
    pub amount: String,
    /// Bech32 pool ID being delegated to.
    pub pool_id: String,
}

/// Created by [`accounts_delegations`](BlockFrostApi::accounts_delegations) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountDelegation {
    /// Epoch in which the delegation becomes active.
    pub active_epoch: Integer,
    /// Hash of the transaction containing the delegation.
    pub tx_hash: String,
    /// Rewards for given epoch in Lovelaces.
    pub amount: String,
    /// Bech32 ID of pool being delegated to.
    pub pool_id: String,
}

/// Created by [`accounts_registrations`](BlockFrostApi::accounts_registrations) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountRegistration {
    /// Hash of the transaction containing the (de)registration certificate.
    pub tx_hash: String,
    /// Action in the certificate.
    pub action: String, // enum: "registered" | "deregistered"
}

/// Created by [`accounts_withdrawals`](BlockFrostApi::accounts_withdrawals) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountWithdrawal {
    /// Hash of the transaction containing the withdrawal.
    pub tx_hash: String,
    /// Withdrawal amount in Lovelaces.
    pub amount: String,
}

/// Created by [`accounts_mirs`](BlockFrostApi::accounts_mirs) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountMir {
    /// Hash of the transaction containing the MIR.
    pub tx_hash: String,
    /// MIR amount in Lovelaces.
    pub amount: String,
}

/// Created by [`accounts_addresses`](BlockFrostApi::accounts_addresses) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountAddress {
    /// Address associated with the stake key.
    pub address: String,
}

/// Created by [`accounts_addresses_assets`](BlockFrostApi::accounts_addresses_assets) method.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountAddressAsset {
    /// The unit of the value.
    ///
    /// Format: Concatenation of asset `policy_id` and hex-encoded `asset_name`.
    pub unit: String,
    /// The quantity of the unit.
    pub quantity: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    test_schema! { test_accounts, Account, r#"
    {
      "stake_address": "stake1ux3g2c9dx2nhhehyrezyxpkstartcqmu9hk63qgfkccw5rqttygt7",
      "active": true,
      "active_epoch": 412,
      "controlled_amount": "619154618165",
      "rewards_sum": "319154618165",
      "withdrawals_sum": "12125369253",
      "reserves_sum": "319154618165",
      "treasury_sum": "12000000",
      "withdrawable_amount": "319154618165",
      "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
    }
    "# }

    test_schema! { test_account_reward, Vec<AccountReward>, r#"
    [
      {
        "epoch": 215,
        "amount": "12695385",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      },
      {
        "epoch": 216,
        "amount": "3586329",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      },
      {
        "epoch": 217,
        "amount": "0",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      },
      {
        "epoch": 218,
        "amount": "1395265",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      }
    ]
    "# }

    test_schema! { test_account_history, Vec<AccountHistory>, r#"
    [
      {
        "active_epoch": 210,
        "amount": "12695385",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      },
      {
        "active_epoch": 211,
        "amount": "22695385",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      }
    ]
    "# }

    test_schema! { test_account_delegation, Vec<AccountDelegation>, r#"
    [
      {
        "active_epoch": 210,
        "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
        "amount": "12695385",
        "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy"
      },
      {
        "active_epoch": 242,
        "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
        "amount": "12691385",
        "pool_id": "pool1kchver88u3kygsak8wgll7htr8uxn5v35lfrsyy842nkscrzyvj"
      }
    ]
    "# }

    test_schema! { test_account_registration, Vec<AccountRegistration>, r#"
    [
      {
        "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
        "action": "registered"
      },
      {
        "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
        "action": "deregistered"
      }
    ]
    "# }

    test_schema! { test_account_withdrawal, Vec<AccountWithdrawal>, r#"
    [
      {
        "tx_hash": "48a9625c841eea0dd2bb6cf551eabe6523b7290c9ce34be74eedef2dd8f7ecc5",
        "amount": "454541212442"
      },
      {
        "tx_hash": "4230b0cbccf6f449f0847d8ad1d634a7a49df60d8c142bb8cc2dbc8ca03d9e34",
        "amount": "97846969"
      }
    ]
    "# }

    test_schema! { test_account_mir, Vec<AccountMir>, r#"
    [
      {
        "tx_hash": "48a9625c841eea0dd2bb6cf551eabe6523b7290c9ce34be74eedef2dd8f7ecc5",
        "amount": "454541212442"
      },
      {
        "tx_hash": "4230b0cbccf6f449f0847d8ad1d634a7a49df60d8c142bb8cc2dbc8ca03d9e34",
        "amount": "97846969"
      }
    ]
    "# }

    test_schema! { test_account_address, Vec<AccountAddress>, r#"
    [
      {
        "address": "addr1qx2kd28nq8ac5prwg32hhvudlwggpgfp8utlyqxu6wqgz62f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sy0f4qd"
      },
      {
        "address": "addr1qys3czp8s9thc6u2fqed9yq3h24nyw28uk0m6mkgn9dkckjf79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9suth4w4"
      },
      {
        "address": "addr1q8j55h253zcvl326sk5qdt2n8z7eghzspe0ekxgncr796s2f79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sjmd35m"
      },
      {
        "address": "addr1q8f7gxrprank3drhx8k5grlux7ene0nlwun8y9thu8mc3yjf79qsdmm5dsknt9ecr5w468r9ey0fxwkdrwh08ly3tu9sls6vnt"
      }
    ]
    "# }

    test_schema! { test_account_address_asset, Vec<AccountAddressAsset>, r#"
    [
      {
        "unit": "d5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc537061636542756433343132",
        "quantity": "1"
      },
      {
        "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
        "quantity": "125"
      }
    ]
    "# }
}
