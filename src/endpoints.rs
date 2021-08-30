use crate::{models, BlockFrostApi, Result};

impl BlockFrostApi {
    pub async fn root(&self) -> Result<models::Root> {
        self.get("/").await
    }

    pub async fn health(&self) -> Result<models::Health> {
        self.get("/health").await
    }

    pub async fn health_clock(&self) -> Result<models::HealthClock> {
        self.get("/health/clock").await
    }

    pub async fn metrics(&self) -> Result<Vec<models::Metric>> {
        self.get("/metrics").await
    }

    pub async fn metrics_endpoints(&self) -> Result<Vec<models::MetricEndpoint>> {
        self.get("/metrics/endpoints").await
    }

    pub async fn accounts(&self, account_address: &str) -> Result<models::Account> {
        let suffix = format!("/accounts/{}", account_address);
        self.get(&suffix).await
    }

    pub async fn accounts_rewards(&self, account_address: &str) -> Result<Vec<models::AccountRewards>> {
        let suffix = format!("/accounts/{}/rewards", account_address);
        self.get(&suffix).await
    }
}
