use crate::{BlockFrostApi, Pagination, Result};
use blockfrost_openapi::models::{
    account_addresses_assets_inner::AccountAddressesAssetsInner,
    account_addresses_content_inner::AccountAddressesContentInner, account_content::AccountContent,
    account_delegation_content_inner::AccountDelegationContentInner,
    account_history_content_inner::AccountHistoryContentInner,
    account_mir_content_inner::AccountMirContentInner,
    account_registration_content_inner::AccountRegistrationContentInner,
    account_reward_content_inner::AccountRewardContentInner,
    account_withdrawal_content_inner::AccountWithdrawalContentInner,
};

impl BlockFrostApi {
    pub async fn accounts(&self, stake_address: &str) -> Result<AccountContent> {
        self.call_endpoint(format!("/accounts/{}", stake_address).as_str())
            .await
    }

    /// Reward history of a specific account.
    pub async fn accounts_rewards(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountRewardContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/rewards", stake_address).as_str(),
            pagination,
        )
        .await
    }

    /// History of a specific account.
    pub async fn accounts_history(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountHistoryContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/history", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_delegations(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountDelegationContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/delegations", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_registrations(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountRegistrationContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/registrations", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_withdrawals(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountWithdrawalContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/withdrawals", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_mirs(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountMirContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/mirs", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_addresses(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountAddressesContentInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/addresses", stake_address).as_str(),
            pagination,
        )
        .await
    }

    pub async fn accounts_addresses_assets(
        &self,
        stake_address: &str,
        pagination: Option<Pagination>,
    ) -> Result<Vec<AccountAddressesAssetsInner>> {
        self.call_paged_endpoint(
            format!("/accounts/{}/addresses/assets", stake_address).as_str(),
            pagination,
        )
        .await
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_accounts() {
        let json_value = json!({
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
        });

        serde_json::from_value::<AccountContent>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_reward() {
        let json_value = json!([
            {
                "epoch": 215,
                "amount": "12695385",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "type": "leader"
            },
            {
                "epoch": 216,
                "amount": "3586329",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "type": "leader"
            },
            {
                "epoch": 217,
                "amount": "0",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "type": "leader"
            },
            {
                "epoch": 218,
                "amount": "1395265",
                "pool_id": "pool1pu5jlj4q9w9jlxeu370a3c9myx47md5j5m2str0naunn2q3lkdy",
                "type": "leader"
            }
        ]);

        serde_json::from_value::<Vec<AccountRewardContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_history() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AccountHistoryContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_delegation() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AccountDelegationContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_registration() {
        let json_value = json!([
            {
                "tx_hash": "2dd15e0ef6e6a17841cb9541c27724072ce4d4b79b91e58432fbaa32d9572531",
                "action": "registered"
            },
            {
                "tx_hash": "1a0570af966fb355a7160e4f82d5a80b8681b7955f5d44bec0dde628516157f0",
                "action": "deregistered"
            }
        ]);

        serde_json::from_value::<Vec<AccountRegistrationContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_withdrawal() {
        let json_value = json!([
            {
                "tx_hash": "48a9625c841eea0dd2bb6cf551eabe6523b7290c9ce34be74eedef2dd8f7ecc5",
                "amount": "454541212442"
            },
            {
                "tx_hash": "4230b0cbccf6f449f0847d8ad1d634a7a49df60d8c142bb8cc2dbc8ca03d9e34",
                "amount": "97846969"
            }
        ]);

        serde_json::from_value::<Vec<AccountWithdrawalContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_mir() {
        let json_value = json!([
            {
                "tx_hash": "48a9625c841eea0dd2bb6cf551eabe6523b7290c9ce34be74eedef2dd8f7ecc5",
                "amount": "454541212442"
            },
            {
                "tx_hash": "4230b0cbccf6f449f0847d8ad1d634a7a49df60d8c142bb8cc2dbc8ca03d9e34",
                "amount": "97846969"
            }
        ]);

        serde_json::from_value::<Vec<AccountMirContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_address() {
        let json_value = json!([
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
        ]);

        serde_json::from_value::<Vec<AccountAddressesContentInner>>(json_value).unwrap();
    }

    #[tokio::test]
    async fn test_account_address_asset() {
        let json_value = json!([
            {
                "unit": "d5e6bf0500378d4f0da4e8dde6becec7621cd8cbf5cbb9b87013d4cc537061636542756433343132",
                "quantity": "1"
            },
            {
                "unit": "b0d07d45fe9514f80213f4020e5a61241458be626841cde717cb38a76e7574636f696e",
                "quantity": "125"
            }
        ]);

        serde_json::from_value::<Vec<AccountAddressesAssetsInner>>(json_value).unwrap();
    }
}
